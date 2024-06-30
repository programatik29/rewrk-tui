use self::{
    client::Client,
    error::{BenchError, BenchResult},
};
use crate::{state::State, Args};
use std::{
    future::Future,
    sync::{mpsc, Arc},
    thread,
    time::Duration,
};
use tokio::time::{sleep_until, Instant};

mod client;
mod error;
mod record;

pub fn start(args: Args, state: Arc<State>) -> BenchResult<()> {
    let (tx, rx) = mpsc::channel();

    thread::spawn(|| {
        tokio::runtime::Builder::new_multi_thread()
            .worker_threads(args.threads)
            .enable_all()
            .build()?
            .block_on(run_benchmark(args, state, tx));

        anyhow::Ok(())
    });

    if rx.recv().is_err() {
        return Err(BenchError::ConnFailed);
    }

    Ok(())
}

async fn run_benchmark(args: Args, state: Arc<State>, tx: mpsc::Sender<()>) {
    let args = Arc::new(args);
    let deadline = state.start + Duration::from_secs(args.duration);

    let client = match Client::connect(&args, &state).await {
        Ok(v) => v,
        Err(_) => return,
    };

    start_request_counter(state.clone());

    tx.send(()).unwrap();

    tokio::spawn(timeout_at(
        deadline,
        connection(args.clone(), Some(client), state.clone()),
    ));

    for _ in 1..args.connections {
        tokio::spawn(timeout_at(
            deadline,
            connection(args.clone(), None, state.clone()),
        ));
    }

    sleep_until(deadline).await;
}

fn start_request_counter(state: Arc<State>) {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_millis(250));

        loop {
            interval.tick().await;

            let total = state.request_total.load();
            let elapsed = state.start.elapsed().as_millis() as u64;
            let per_millisecond = total / elapsed;

            state.request_second.set(per_millisecond * 1000);
        }
    });
}

async fn connection(args: Arc<Args>, mut stream: Option<Client>, state: Arc<State>) {
    loop {
        if let Err(e) = try_connection(&args, &mut stream, &state).await {
            state.errors.lock().add_error(e);
        }
    }
}

async fn try_connection(
    args: &Args,
    client: &mut Option<Client>,
    state: &Arc<State>,
) -> BenchResult<()> {
    let mut client = match client.take() {
        Some(v) => v,
        None => Client::connect(args, state).await?,
    };

    loop {
        client.send_request().await?;

        state.request_total.add(1);
    }
}

async fn timeout_at<F: Future>(instant: Instant, future: F) {
    tokio::select! {
        biased;
        _ = sleep_until(instant) => (),
        _ = future => (),
    }
}
