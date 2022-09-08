use shindanmaker::get;
use std::env::args;

#[tokio::main]
async fn main() {
    let name = args().nth(1).unwrap();
    let card = get(&name).await.unwrap();

    println!("{card}");
}
