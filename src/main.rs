use job::job;
use tokio::{task::JoinSet, time::{sleep, Duration}};

mod job;

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    // This is running on a core thread.
    let mut set = JoinSet::new();
    
    for i in 0..u32::MAX {
        let k = i;
        let _ =  set.spawn(async move {
            let _ = job(k).await;
        });
    }
    //sleep(Duration::from_secs(10)).await;

    while let Some(res) = set.join_next().await {
        let _ = res.unwrap();
    }
    // We can wait for the blocking task like this:
    // If the blocking task panics, the unwrap below will propagate the
    // panic.
}