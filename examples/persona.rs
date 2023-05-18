use shindanmaker::get_persona;
use std::env::args;

#[tokio::main]
async fn main() {
    let name = args().next().unwrap();
    let card = get_persona(&name).await.unwrap();

    println!("{card}");
}
