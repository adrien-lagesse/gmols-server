use public_search;
use tokio;

#[tokio::main]
async fn main() {
    public_search::https::test().await;
}
