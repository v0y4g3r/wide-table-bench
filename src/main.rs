// Copyright 2024 Greptime Team
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::args::Args;
use crate::data_source::Batch;
use clap::Parser;
use greptimedb_client::api::v1::column::Values;
use greptimedb_client::api::v1::{Column, ColumnDataType, InsertRequest, SemanticType};
use greptimedb_client::{Client, Database};
use log::{error, info, warn, LevelFilter};
use tokio::time::Instant;

mod args;
mod data_source;

#[tokio::main]
async fn main() {
    env_logger::builder().filter_level(LevelFilter::Info).init();
    let args = Args::parse();

    info!("Using args: {:?}", args);
    let client = build_client(&args.endpoint);
    let inserter = client.streaming_inserter().unwrap();

    let mut source = data_source::build_data_source(args.batch_size, args.column_num);
    let mut rows_inserted = 0;

    let start = Instant::now();
    let mut percent = 0;

    while let Some(batch) = source.next() {
        let ts = data_source::ts_micros_col(args.batch_size);
        let rows = ts.len();
        if let Some(req) = rows_to_insert_request(&args.table_name, ts, batch) {
            if let Err(e) = inserter.insert(vec![req]).await {
                error!("Failed to execute insert: {e:?}");
                break;
            } else {
                rows_inserted += rows;
                let current_percent = (rows_inserted * 100) / args.max_rows;
                if current_percent > percent {
                    info!("Rows inserted: {} ({}%)", rows_inserted, current_percent);
                    percent = current_percent;
                }
                if rows_inserted >= args.max_rows {
                    break;
                }
            }
        } else {
            warn!("Failed to build request");
        }
    }
    inserter.finish().await.unwrap();
    let elapsed = Instant::now().duration_since(start).as_millis() as usize;
    info!(
        "Total rows inserted: {}, elapsed: {}ms, TPS: {}, PPS: {}",
        rows_inserted,
        elapsed,
        rows_inserted * 1000 / elapsed,
        (rows_inserted * args.column_num) * 1000 / elapsed
    );
}

fn rows_to_insert_request(table_name: &str, ts: Vec<i64>, batch: Batch) -> Option<InsertRequest> {
    let (vin_col, field_cols) = batch;
    if vin_col.len() == 0 {
        return None;
    }

    let rows = ts.len();

    let mut columns = Vec::with_capacity(field_cols.len() + 2);
    columns.push(Column {
        column_name: "ts".to_string(),
        semantic_type: SemanticType::Timestamp as i32,
        values: Some(Values {
            timestamp_microsecond_values: ts,
            ..Default::default()
        }),
        datatype: ColumnDataType::TimestampMicrosecond as i32,
        ..Default::default()
    });

    columns.push(Column {
        column_name: "vin".to_string(),
        semantic_type: SemanticType::Tag as i32,
        values: Some(Values {
            i8_values: vin_col,
            ..Default::default()
        }),
        datatype: ColumnDataType::Int8 as i32,
        ..Default::default()
    });

    columns.extend(field_cols.into_iter().enumerate().map(|(idx, col)| Column {
        column_name: format!("col_{}", idx),
        semantic_type: SemanticType::Field as i32,
        values: Some(Values {
            f32_values: col,
            ..Default::default()
        }),
        datatype: ColumnDataType::Float32 as i32,
        ..Default::default()
    }));

    Some(InsertRequest {
        table_name: table_name.to_string(),
        columns,
        row_count: rows as u32,
    })
}

fn build_client(endpoint: &str) -> Database {
    let grpc_client = Client::with_urls(vec![endpoint]);
    Database::new_with_dbname("public".to_string(), grpc_client)
}
