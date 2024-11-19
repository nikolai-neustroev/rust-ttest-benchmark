use std::time::Instant;
use std::io::Read;
use npy::NpyData;
use std::fs::File;
use std::error::Error;
use csv::Writer;
use indicatif::{ProgressBar, ProgressStyle};  // Import indicatif for progress bar

mod ttest_ind;

fn main() -> Result<(), Box<dyn Error>> {
    let mut a_buf = vec![];
    File::open("data_a.npy")?.read_to_end(&mut a_buf)?;
    let a_array: NpyData<f64> = NpyData::from_bytes(&a_buf)?;
    
    let mut b_buf = vec![];
    File::open("data_b.npy")?.read_to_end(&mut b_buf)?;
    let b_array: NpyData<f64> = NpyData::from_bytes(&b_buf)?;
    
    let a = a_array.to_vec();
    let b = b_array.to_vec();

    let mut runtimes = Vec::new();

    let mut t_stat = 0.0;
    let mut p_value = 0.0;

    let pb = ProgressBar::new(10_000);
    pb.set_style(ProgressStyle::default_bar()
        .template("{msg} {bar:40.cyan/blue} {pos}/{len} [{elapsed}<{eta}]")
        .progress_chars("##-"));

    pb.set_message("Running t-tests");

    for _ in 0..10_000 {
        let start = Instant::now();
        let (t, p) = ttest_ind::ttest_ind(&a, &b, false, "two-sided");
        let duration = start.elapsed();
        runtimes.push(duration.as_secs_f64());

        t_stat = t;
        p_value = p;

        pb.inc(1);
    }

    pb.finish_with_message("Completed t-tests");

    println!("Rust t-statistic: {:.4}, p-value: {:.4}", t_stat, p_value);
    println!("Rust last execution time: {:.6} seconds", runtimes.last().unwrap());

    let mut wtr = Writer::from_path("rust_runtimes.csv")?;
    wtr.write_record(&["runtime"])?;
    for runtime in runtimes {
        wtr.write_record(&[runtime.to_string()])?;
    }
    wtr.flush()?;

    Ok(())
}
