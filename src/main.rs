use workers::Worker;

pub mod models;
pub mod workers;
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let worker = Worker::default();
    let media = worker.search("gato de botas 2").await?;

    Ok(())
}
