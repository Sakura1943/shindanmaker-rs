# Shindanmaker Client

A library to visit https://en.shindanmaker.com/917962 or https://en.shindanmaker.com/{page_id}.

## 📔 Usage

Add dependencies to `Cargo.toml`:

```toml
tokio = { version = "1.28.1", features = ["macros", "rt-multi-thread"] }
shindanmaker-rs = { version = "0.3.1" }
```



## 🤖 Example CLI

Use the following command to fetch and print diagnosis information:

```bash
$ cargo run --example persona <name>
```



## 🛠  Features

- `serde`: *Serialize* and *Deserialize* support for `Card`.



## 💳 License

MIT license ([LICENSE](./LICENSE) or https://opensource.org/licenses/MIT)
