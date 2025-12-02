use anyhow::Result;
use std::{time};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use rustle::{extract, transform, load};


pub fn run(input: &str, db_path: &str, buffer_size: usize, csv_headers: bool, batch_size: usize) -> Result<()> {
    println!("Running ETL:");
    println!("  input: {}", input);
    println!("  db: {}", db_path);

    /* Create a progress bar */
    let pb = ProgressBar::new_spinner();
    /* Automatically update progress bar every 120ms */
    pb.enable_steady_tick(time::Duration::from_millis(120));
    pb.set_style(
        ProgressStyle::default_spinner()
            .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧"])
            .template("{spinner:.green} {msg}")?
    );

    /* Create Sender / Receiver pairs for bounded channels */
    let (sender_raw, receiver_raw) = flume::bounded(buffer_size);
    let (sender_transformed, receiver_transformed) = flume::bounded(buffer_size);

    /* Record the moment of initiation */
    let start = time::Instant::now();

    pb.set_message("Extracting");
    /* Create reader thread */
    let reader = extract::spawn_reader(input, sender_raw, csv_headers);

    pb.set_message("Transforming");
    /* Create transform worker thread pool */
    let transformers = transform::spawn_workers(receiver_raw, sender_transformed);

    pb.set_message("Loading");
    /* Create writer thread */
    let writer = load::spawn_batch_writer(db_path.to_string(), receiver_transformed, batch_size);

    /* Join threads - ensure that threads finish execution */
    reader.join().unwrap();
    transformers.join().unwrap();
    let (ok, fail) = writer.join()
        .unwrap()
        .unwrap();

    pb.finish_with_message("ETL completed");

    /* Record the moment of termination */
    let duration = start.elapsed().as_secs_f64();
    /* Calculate rows per second */
    let rps = ok as f64 / duration;

    println!("\n{}", "================ ETL SUMMARY ================".bold());
    println!("{} {:.2}s", "Total duration:".cyan(), duration);
    println!("{} {:.0}", "Rows/s:".cyan(), rps);
    println!("{} {}", "Successful rows:".green(), ok.to_string().green().bold());
    println!("{} {}", "Failed rows:".red(), fail.to_string().red().bold());
    println!("{} {}", "Total records processed:".yellow(), (ok + fail).to_string().yellow().bold());
    println!("{}", "==============================================".bold());

    Ok(())
}
