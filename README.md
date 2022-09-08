# Shindanmaker Client

A library to visit https://en.shindanmaker.com/917962 .

## 📔 Usage

Add dependencies to `Cargo.toml`:

```toml
tokio = { version = "1.21.0", features = ["macros", "rt-multi-thread"] }
shindanmaker-rs = { version = "0.2.4" }
```



## 🤖 Example CLI

Use the following command to fetch and print diagnosis information:

```bash
$ cargo run --example cli <name>
```



## 🛠  Features

- `serde`: *Serialize* and *Deserialize* support for `Card`.



## 💳 License

MIT license ([LICENSE](./LICENSE) or https://opensource.org/licenses/MIT)
