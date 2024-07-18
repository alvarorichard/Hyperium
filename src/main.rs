use workers::Worker;

pub mod models;
pub mod workers;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let worker = Worker::default();

    worker.search("asdasda").await?;
    Ok(())
}
