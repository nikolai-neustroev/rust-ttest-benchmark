use statrs::distribution::{StudentsT, ContinuousCDF};

pub fn ttest_ind(a: &[f64], b: &[f64], equal_var: bool, alternative: &str) -> (f64, f64) {
    let n1 = a.len() as f64;
    let n2 = b.len() as f64;

    let mean1 = a.iter().sum::<f64>() / n1;
    let mean2 = b.iter().sum::<f64>() / n2;

    let var1 = a.iter().map(|x| (x - mean1).powi(2)).sum::<f64>() / (n1 - 1.0);
    let var2 = b.iter().map(|x| (x - mean2).powi(2)).sum::<f64>() / (n2 - 1.0);

    let t_stat: f64;
    let df: f64;

    if equal_var {
        let pooled_var = ((n1 - 1.0) * var1 + (n2 - 1.0) * var2) / (n1 + n2 - 2.0);
        let standard_error = (pooled_var * (1.0 / n1 + 1.0 / n2)).sqrt();
        t_stat = (mean1 - mean2) / standard_error;
        df = n1 + n2 - 2.0;
    } else {
        let s1_n1 = var1 / n1;
        let s2_n2 = var2 / n2;
        let standard_error = (s1_n1 + s2_n2).sqrt();
        t_stat = (mean1 - mean2) / standard_error;
        df = ((s1_n1 + s2_n2).powi(2))
            / ((s1_n1.powi(2) / (n1 - 1.0)) + (s2_n2.powi(2) / (n2 - 1.0)));
    }

    let dist = StudentsT::new(0.0, 1.0, df).unwrap();
    let p_value = match alternative {
        "two-sided" => 2.0 * (1.0 - dist.cdf(t_stat.abs())),
        "greater" => 1.0 - dist.cdf(t_stat),
        "less" => dist.cdf(t_stat),
        _ => panic!("Invalid alternative hypothesis"),
    };

    (t_stat, p_value)
}
