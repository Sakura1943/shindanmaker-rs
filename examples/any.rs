use shindanmaker::get_by_id;
use std::env::args;

#[tokio::main]
async fn main() {
    let mut args = args().skip(1);
    let page_id: u64 = args
            .next()
            .unwrap()
            .parse()
            .unwrap();
    let name = args.next().unwrap();
    let info = get_by_id(page_id, &name).await.unwrap();

    println!("{info}");
}
