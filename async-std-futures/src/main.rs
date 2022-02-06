use async_std::task;

use random_seed_future::{random_seed, RandomSeedFuture};

pub fn main() {
    task::block_on(async {
        let _data = RandomSeedFuture::new(1_000_000_000).await;
        let _other_data = random_seed().await;
        let _both = futures::future::join_all(vec![random_seed(), random_seed()]).await;
    });
}
// #[async_std::main]
// pub async fn main() {
//     let _data = RandomSeedFuture::new(1_000_000_000).await;
//     let _other_data = random_seed().await;
//     let _both = futures::future::join_all(vec![random_seed(), random_seed()]).await;
// }
