use random_seed_future::{random_seed, RandomSeedFuture};
use tokio::runtime::Runtime;

fn main() {
    let rt = Runtime::new().unwrap();

    rt.block_on(async {
        let _other = tokio::spawn(random_seed());
        let _data = RandomSeedFuture::new(1_000_000_000).await;
        let _other_data = random_seed().await;
        let _both = futures::future::join_all(vec![random_seed(), random_seed()]).await;
    });
    println!("Finished");
}


// #[tokio::main]
// pub async fn main() {
//     let _data = RandomSeedFuture::new(1_000_000_000).await;
//     let _other_data = random_seed().await;
//     let _both = futures::future::join_all(vec![random_seed(), random_seed()]).await;
// }
