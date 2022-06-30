use anyhow::Result;
use crossbeam::channel;
use ed2k::Ed2k;
use std::collections::HashMap;
use std::env;
use std::num::NonZeroUsize;
use std::path::PathBuf;
use std::thread;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "ed2krs",
    about = "Calculate the eD2k link for the given set of files."
)]
struct Opt {
    /// Use legacy hashing algorithm
    #[structopt(short = "l", long = "legacy")]
    legacy: bool,

    /// Number of threads to use. Uses the number of CPU cores by default
    #[structopt(short = "t", long = "num-threads")]
    num_threads: Option<NonZeroUsize>,

    /// Files to hash
    #[structopt(parse(from_os_str), required = true)]
    files: Vec<PathBuf>,
}

fn main() -> Result<()> {
    let opts = Opt::from_args();

    // Create communication channels
    let (job_tx, job_rx) = channel::unbounded();
    let (result_tx, result_rx) = channel::unbounded();

    // Spawn a thread pool
    let max_num_threads = opts
        .num_threads
        .map(NonZeroUsize::get)
        .unwrap_or_else(|| num_cpus::get().into());
    for _ in 0..max_num_threads.min(env::args().len() - 1) {
        let rx = job_rx.clone();
        let tx = result_tx.clone();

        if opts.legacy {
            thread::spawn(move || {
                for (i, path) in rx {
                    tx.send((i, Ed2k::from_path_legacy(&path))).unwrap();
                }
            });
        } else {
            thread::spawn(move || {
                for (i, path) in rx {
                    tx.send((i, Ed2k::from_path(&path))).unwrap();
                }
            });
        }
    }
    drop(job_rx);
    drop(result_tx);

    // Fill pool with jobs
    for job in opts.files.into_iter().enumerate() {
        job_tx.send(job)?;
    }
    drop(job_tx);

    let mut next_idx = 0;
    let mut out_of_order = HashMap::new();
    for (i, ed2k) in result_rx {
        // Buffer results to make sure they are printed in input order
        out_of_order.insert(i, ed2k);
        while let Some(ed2k) = out_of_order.remove(&next_idx) {
            println!("{}", ed2k?);
            next_idx += 1;
        }
    }

    Ok(())
}
