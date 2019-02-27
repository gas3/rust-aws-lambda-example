# rust-aws-lambda-example

Build:
```bash
cargo build --release --target x86_64-unknown-linux-musl
```

Package:
```bash
cp ./target/x86_64-unknown-linux-musl/release/rust-aws-lambda-user-exists ./bootstrap && zip lambda.zip bootstrap
```

Test:
```bash
docker run --rm -v "$PWD":/var/task lambci/lambda:provided handler '{"username": "tizio90"}'
```
