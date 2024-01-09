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

use clap::Parser;

#[derive(Debug, Default, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Database gRPC endpoint
    #[clap(short, long, default_value = "127.0.0.1:4001")]
    pub endpoint: String,

    /// Database batch size
    #[clap(short, long, default_value_t = 10)]
    pub batch_size: usize,

    /// Max rows to insert
    #[clap(long, default_value_t = 10000)]
    pub max_rows: usize,

    /// Insertion concurrency
    #[clap(long, default_value_t = 4)]
    pub concurrency: usize,

    /// Field column num of table.
    #[clap(long, default_value_t = 1200)]
    pub column_num: usize,

    #[clap(long, default_value = "bench")]
    pub table_name: String,
}
