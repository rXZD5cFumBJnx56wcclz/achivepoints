use achivepoints::app::App;
use tokio::main;

#[main]
async fn main() {
    App.run().await.unwrap();
}
