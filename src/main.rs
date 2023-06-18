
use hyper::Client;
use hyper::body::HttpBody as _;
use tokio::io::{stdout, AsyncWriteExt as _};






#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {

    
    let client = Client::new();

    let uri = "http://127.0.0.1:8080".parse()?;


    let mut resp = client.get(uri).await?;

    

    while let Some(chunk) = resp.body_mut().data().await {
        stdout().write_all(&chunk?).await?
    }



    Ok(())
}

