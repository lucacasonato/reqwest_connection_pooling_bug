use futures::StreamExt;
use reqwest::Client;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let client = Client::new();

    let mut futures = futures::stream::FuturesOrdered::new();

    for i in 0..100 {
        futures.push(loop_(i, &client));
    }

    while let Some(_) = futures.next().await {}
}

async fn loop_(i: usize, client: &Client) {
    for _ in 0..20 {
        client
            .get("https://deno-website2.now.sh/")
            .send()
            .await
            .unwrap()
            .error_for_status()
            .unwrap();
    }
    println!("task {} done", i);
}
