use server::WebServer;
mod schemas;
mod server;
mod database;

#[async_std::main]
async fn main() -> tide::Result<()> {
    let server= WebServer::new().await;
    Ok(())
}
