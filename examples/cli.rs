use shindanmaker::{
    get,
    // get_else
};
use std::env::args;

#[tokio::main]
async fn main() {
    let name = args().nth(1).unwrap();
    let card = get(&name).await.unwrap();
    // let card = get_else(1023457, &name).await.unwrap();
    println!("{card}");
}
