# Shindanmaker in Rust
A library to visit `https://en.shindanmaker.com/917962`, and use method `get` to achieve data.

## 📔 Usage
Make sure you activated the shindanmaker-rs crate on Cargo.toml
```toml
tokio = { version = "1.21.0", features = ["full"] }
shindanmaker-rs = { version = "0.1.1" }
```
Then, on your main.rs:
```rust
use shindanmaker::get;

#[tokio::main]
async fn main() {
  let result = get("demo").await.unrwap();
  println!("{}", result);
}
```
or
```rust
use shindanmaker::get;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
  let result = get("demo").await?;
  println!("{}", result);
  Ok(())
}
```

## 📔 license
MIT license ([LICENSE](./LICENSE) or https://opensource.org/licenses/MIT)
