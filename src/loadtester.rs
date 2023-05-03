use crate::client;
use std::time::Instant;
use tokio::runtime::Builder;
use tokio::task;
use crate::datastore;
use crate::datastore::DataStoreService;

pub fn loadtest(url: &str, num_requests: usize) {
    let rt = Builder::new_multi_thread()
        .worker_threads(12)
        .enable_all()
        .build()
        .unwrap();
    rt.block_on(loadtest_main(url, num_requests));
}

async fn loadtest_main(url: &str, num_requests: usize) {
    let ds_client = client::DataStoreClient::new(url);

    let start = Instant::now();
    let mut handles = vec![];

    for _ in 0..num_requests {
        let ds_client = ds_client.clone();
        let handle = task::spawn(async move {
            let command = datastore::Command::Set {
                key: "foo".to_string(),
                value: "bar".to_string(),
            };
            let result = ds_client.execute(command);
            if result.is_err() {
                println!("error: {:?}", result);
            }
            //println!("{:?}", result);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.await;
    }

    let elapsed = start.elapsed();
    // print num requests and time and throughput
    println!(
        "completed {} requests in {:?} seconds, throughput: {} requests/second",
        num_requests,
        elapsed,
        num_requests as f64 / elapsed.as_secs_f64()
    );
}
