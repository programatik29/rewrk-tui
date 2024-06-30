use anyhow::anyhow;
use clap::Parser;
use std::{ops::Deref, str::FromStr};
use url::Url;

#[derive(Debug, Parser)]
pub struct InputArgs {
    /// Thread count [default: CPU core count]
    #[arg(short, long)]
    threads: Option<usize>,

    /// Connection count [default: Thread count x20]
    #[arg(short, long)]
    connections: Option<usize>,

    /// Benchmark duration in seconds
    #[arg(short, long)]
    pub duration: u64,

    /// Target URL
    pub target: Target,
}

#[derive(Debug)]
pub struct Args {
    pub threads: usize,
    pub connections: usize,
    inner: InputArgs,
}

impl Args {
    pub fn parse() -> Self {
        let input = InputArgs::parse();
        let threads = input.threads.unwrap_or_else(num_cpus::get);
        let connections = input.connections.unwrap_or(threads * 20);

        Self {
            threads,
            connections,
            inner: input,
        }
    }
}

impl Deref for Args {
    type Target = InputArgs;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

#[derive(Debug, Clone)]
pub struct Target {
    pub host: String,
    pub port: u16,
    pub path: String,
}

impl FromStr for Target {
    type Err = anyhow::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let url = Url::parse(input)?;
        let host = url
            .host_str()
            .ok_or_else(|| anyhow!("target doesn't have host"))?
            .to_owned();
        let port = url.port().unwrap_or(80);
        let path = url.path().to_owned();

        Ok(Self { host, port, path })
    }
}
