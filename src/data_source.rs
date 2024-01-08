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

use log::info;
use rand::rngs::ThreadRng;
use rand::Rng;
use std::sync;
use std::time::{SystemTime, UNIX_EPOCH};

pub type FieldCol = Vec<f32>;
pub type VinCol = Vec<i32>;
pub type Batch = (VinCol, Vec<FieldCol>);

/// Builds data source /w given config.
pub fn build_data_source(batch: usize, columns: usize) -> impl Iterator<Item = Batch> {
    let (tx, rx) = sync::mpsc::sync_channel(1024);

    std::thread::spawn(move || {
        let mut rng = rand::thread_rng();
        loop {
            let vin = random_vin_col(batch, &mut rng);
            let fields = (0..columns).map(|_| random_row(batch, &mut rng)).collect();

            if let Err(_) = tx.send((vin, fields)) {
                info!("Data source exit");
                break;
            }
        }
    });

    rx.into_iter()
}

/// ### Safety
/// Panics when system jump back.
pub fn ts_micros_col(batch_size: usize) -> Vec<i64> {
    let ts = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_micros() as i64;

    (0..batch_size).map(|delta| ts + delta as i64).collect()
}

#[inline]
pub fn random_vin_col(batch_size: usize, rng: &mut ThreadRng) -> Vec<i32> {
    (0..batch_size).map(|_| rng.gen_range(0..200)).collect()
}

#[inline]
fn random_row(col: usize, rng: &mut ThreadRng) -> Vec<f32> {
    (0..col).map(|_| rng.gen_range(0.0..120.0)).collect()
}

#[cfg(test)]
mod tests {
    use crate::data_source::build_data_source;

    #[test]
    fn test_gen() {
        let mut source = build_data_source(1, 10);
        let (vin, fields) = source.next().unwrap();

        assert_eq!(1, vin.len());
        assert_eq!(10, fields.len());
    }
}
