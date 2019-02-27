# rust-aws-lambda-example
This is an example of an AWS Lambda function written in Rust. The function checks the existence of an user, retrieving the list of users from AWS S3 using Rusoto.

This repository is based on [rust-aws-lambda](https://github.com/kkostov/rust-aws-lambda).

Build:
```bash
cargo build --release --target x86_64-unknown-linux-musl
```

Package:
```bash
cp ./target/x86_64-unknown-linux-musl/release/rust-aws-lambda-user-exists ./bootstrap && zip lambda.zip bootstrap config.toml
```

Test:
```bash
docker run --env-file credentials --rm -v "$PWD":/var/task lambci/lambda:provided handler '{"username": "tizio90"}'
```

Config file:
```toml
s3-bucket = "your_bucket"
s3-key = "your_json_path"
```

Users file:
```json
[
  {
    "username": "tizio90",
    "name": "Tizio"
  },
  {
    "username": "caio83",
    "name": "Caio"
  },
  {
    "username": "sempronio85",
    "name": "Sempronio"
  }
]
```