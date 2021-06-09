use std::borrow::Cow;

use futures::StreamExt;
use reqwest::Client;
use tracing::Level;
use tracing_subscriber::FmtSubscriber;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::DEBUG) // Or `Level::TRACE` for even more detail.
        .finish();
    tracing::subscriber::set_global_default(subscriber).unwrap();

    #[cfg(not(feature = "no-pool"))]
    let client = Client::new();

    let mut futures = futures::stream::FuturesOrdered::new();

    for i in 0..100 {
        #[cfg(feature = "no-pool")]
        let client = Client::new();
        #[cfg(feature = "no-pool")]
        let client = Cow::Owned(client);
        #[cfg(not(feature = "no-pool"))]
        let client = Cow::Borrowed(&client);
        futures.push(loop_(i, client));
    }

    futures.collect().await
}

async fn loop_(i: usize, client: Cow<'_, Client>) {
    for _ in 0..20 {
        client
            .get("https://deno-website2.now.sh/")
            .send()
            .await
            .map_err(|err| err.to_string())
            .unwrap()
            .error_for_status()
            .unwrap();
    }
    println!("task {} done", i);
}
