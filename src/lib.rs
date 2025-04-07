use std::error::Error;
pub mod handlers;
pub mod services;
pub mod types;
pub mod routes;

pub async fn run() -> Result<(), Box<dyn Error>> {
    let db = Vec::new();

    let app = routes::create_routes(db).await?;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("listening on {}", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}