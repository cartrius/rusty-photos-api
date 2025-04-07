use axum::{routing::get, Router, extract::Query};
use aws_config::BehaviorVersion;
use aws_sdk_s3::Client as S3Client;
use serde::Deserialize;
use std::net::SocketAddr;
use std::error::Error;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load AWS config
    let shared_config = aws_config::defaults(BehaviorVersion::latest()).load().await;
    let s3_client = S3Client::new(&shared_config);

    // Build Axum router
    let app = Router::new().route(
        "/get-upload-url",
        get({
            let s3_client = s3_client.clone();
            move |query: Query<GetUploadUrlParams>| {
                get_upload_url_handler(query, s3_client.clone())
            }
        }),
    );

    // Define the port
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Rusty Photo API listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
