use anyhow::Result;
use crossbeam::channel;
use ed2k::Ed2kLegacy;
use std::collections::HashMap;
use std::env;
use std::thread;

fn main() -> Result<()> {
    // Create communication channels
    let (job_tx, job_rx) = channel::unbounded();
    let (result_tx, result_rx) = channel::unbounded();

    // Spawn a thread pool
    for _ in 0..num_cpus::get().min(env::args().len() - 1) {
        let rx = job_rx.clone();
        let tx = result_tx.clone();
        thread::spawn(move || {
            for (i, path) in rx {
                tx.send((i, Ed2kLegacy::from_path(&path))).unwrap();
            }
            drop(tx);
        });
    }
    drop(job_rx);
    drop(result_tx);

    // Fill pool with jobs
    for job in env::args().skip(1).enumerate() {
        job_tx.send(job)?;
    }
    drop(job_tx);

    // Some wacky logic to make sure hashes are output in order
    let mut next_idx = 0;
    let mut out_of_order = HashMap::new();
    for (i, ed2k) in result_rx {
        if i == next_idx {
            println!("{}", ed2k?);
            next_idx += 1;

            while let Some(ed2k) = out_of_order.remove(&next_idx) {
                println!("{}", ed2k?);
                next_idx += 1;
            }
        } else {
            out_of_order.insert(i, ed2k);
        }
    }

    Ok(())
}
