#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    directoffice_server::run().await
}

