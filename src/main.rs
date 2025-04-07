use user_project::run;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    match run().await {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
