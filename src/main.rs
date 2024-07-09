use job::job;
use sysinfo::{MemoryRefreshKind, System};
use tokio::{task::JoinSet, time::{sleep, timeout, Duration, Instant}};

mod job;

#[tokio::main(flavor = "multi_thread")]
//#[tokio::main]
async fn main() {
    // This is running on a core thread.
    let mut set = JoinSet::new();
    let mut sys = System::new_all();
    let mut i = 1;

    while i < u32::MAX {
        let k = i;
        //if i%100 == 0 {
            sys.refresh_memory_specifics(MemoryRefreshKind::new().with_ram());
            //}
            if sys.free_memory() > 1000000 {
                let _ =  set.spawn(async move {
                    let _ = job(k).await;
                });
                i+=1;
            }
            else {
                println!("SLEEP");
                sleep(Duration::from_secs(1)).await;
                println!("WAKEUP");
            }
    }

    while let Some(res) = set.join_next().await {
        let _ = res.unwrap();
    }
    // We can wait for the blocking task like this:
    // If the blocking task panics, the unwrap below will propagate the
    // panic.
}