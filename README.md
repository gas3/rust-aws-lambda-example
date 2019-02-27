# rust-aws-lambda-example

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
s3-bucket = "cuebiq-sand"
s3-key = "tmp/mgrandi/users.json"
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