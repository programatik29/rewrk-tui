use crate::Args;

#[tokio::main]
pub async fn start(args: Args) -> anyhow::Result<()> {
    let thread_count = args.threads.unwrap_or_else(num_cpus::get);
    let connection_count = args.connections.unwrap_or(thread_count * 20);

    Ok(())
}
