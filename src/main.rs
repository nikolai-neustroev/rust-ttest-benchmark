use std::time::Instant;
use std::io::Read;
use npy::NpyData;
use std::fs::File;
mod ttest_ind;

fn main() {
    let mut a_buf = vec![];
    File::open("data_a.npy").unwrap().read_to_end(&mut a_buf).unwrap();
    let a_array: NpyData<f64> = NpyData::from_bytes(&a_buf).unwrap();

    let mut b_buf = vec![];
    File::open("data_b.npy").unwrap().read_to_end(&mut b_buf).unwrap();
    let b_array: NpyData<f64> = NpyData::from_bytes(&b_buf).unwrap();

    let a = a_array.to_vec();
    let b = b_array.to_vec();

    let start = Instant::now();
    let (t_stat, p_value) = ttest_ind::ttest_ind(&a, &b, false, "two-sided");
    let duration = start.elapsed();

    println!("Rust t-statistic: {:.4}, p-value: {:.4}", t_stat, p_value);
    println!("Rust execution time: {:.6} seconds", duration.as_secs_f64());
}
