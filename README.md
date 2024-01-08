# GreptimeDB wide table benchmark

## Build from source

```bash
cargo build --release
```

## Usage

```bash

./target/release/wide-table-bench --help
Usage: wide-table-bench [OPTIONS]

Options:
  -e, --endpoint <ENDPOINT>        Database gRPC endpoint [default: 127.0.0.1:4001]
  -b, --batch-size <BATCH_SIZE>    Database batch size [default: 10]
      --max-rows <MAX_ROWS>        Max rows to insert [default: 10000]
      --concurrency <CONCURRENCY>  Insertion concurrency [default: 4]
      --column-num <COLUMN_NUM>    Field column num of table [default: 1600]
      --table-name <TABLE_NAME>    [default: bench]
  -h, --help                       Print help
  -V, --version                    Print version
```