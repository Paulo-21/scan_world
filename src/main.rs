use job::job;
use sysinfo::{MemoryRefreshKind, System};
use tokio::{time::{sleep, Duration, Instant}};
use std::sync::{atomic::{AtomicU32, Ordering}, Arc};

mod job;

#[tokio::main(flavor = "multi_thread")]
//#[tokio::main]
async fn main() {
    // This is running on a core thread.
    let mut sys = System::new_all();
    let mut sys2 = System::new_all();
    let mut i_org : u32 = 1;
    //let raw = &i_org as *const u32;
    let shared = Arc::new(AtomicU32::new(1));
    let s = shared.clone();

    let handle = tokio::runtime::Handle::current();
    let runtime_monitor = tokio_metrics::RuntimeMonitor::new(&handle);
    let limit_memory =  (sys.total_memory() as f64 * 0.92 ) as u64;
    // print runtime metrics every 500ms
    let frequency = std::time::Duration::from_millis(500);
    /*tokio::spawn(async move {
        let start = Instant::now();
        let pr = s;
        for metrics in runtime_monitor.intervals() {
            sys2.refresh_memory_specifics(MemoryRefreshKind::new().with_ram());
            println!("------------------------------------------");
            println!("Metrics = {:?}", metrics);
            println!("Tot {}, used : {}", sys2.total_memory(), sys2.used_memory());
            let value = pr.load(Ordering::Relaxed);
            println!(" {} / {} : {} % ", value , u32::MAX, value as f64 / u32::MAX as f64);
            if start.elapsed().as_secs() > 0 {
                println!("{} r/s", value as u64 / start.elapsed().as_secs());
            }
            tokio::time::sleep(frequency).await;
        }
    });*/
    while i_org < u32::MAX {
        let k = i_org;
        sys.refresh_memory_specifics(MemoryRefreshKind::new().with_ram());
        if sys.used_memory() < limit_memory {
            tokio::task::spawn(async move {
                let _ = job(k).await;
            });
            i_org +=1;
            shared.store(i_org, Ordering::Relaxed);
        }
        else {
            println!("SLEEP");
            sleep(Duration::from_millis(500)).await;
            println!("WAKEUP");
        }
    }

}
