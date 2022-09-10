use shindanmaker::get_persona;
use std::env::args;

#[tokio::main]
async fn main() {
    let name = args().nth(1).unwrap();
    let card = get_persona(&name).await.unwrap();

    println!("{card}");
}
