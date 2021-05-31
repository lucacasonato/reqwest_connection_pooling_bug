use std::borrow::Cow;

use futures::StreamExt;
use reqwest::Client;

#[tokio::main(flavor = "current_thread")]
async fn main() {
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

    while let Some(_) = futures.next().await {}
}

async fn loop_<'a>(i: usize, client: Cow<'a, Client>) {
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
