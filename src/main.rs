mod animate;
mod api;
mod app;
mod cli;
mod dev;
mod error;
mod models;
mod req;
mod storage;
mod view;

#[tokio::main]
async fn main() {
    if let Err(e) = app::run().await {
        eprintln!("{e}");
    }
}
