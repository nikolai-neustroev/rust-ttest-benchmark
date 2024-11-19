# rust-ttest-benchmark

This is a benchmark script for t-test runtime evaluation in Rust.

Run with:

```
cargo run --release
```

The t-test runs 10 000 times over the same dataset.

## Statistical Analysis of Runtimes

- Python mean runtime: 0.00556 seconds
- Rust mean runtime: 0.00350 seconds
- Python standard deviation: 0.00066 seconds
- Rust standard deviation: 0.00025 seconds
- p-value (t-test): 0.0

Rust is significantly faster than Python for this benchmark, with a mean runtime approximately 37% lower than Python's. Rust's runtimes are also more consistent (much smaller standard deviation). The small p-value indicates that the difference in runtimes is statistically significant.

![Histogram](https://private-user-images.githubusercontent.com/38642966/387624523-34dc9f8c-6ecd-4e44-9ca2-1629f33f12f1.png?jwt=eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJnaXRodWIuY29tIiwiYXVkIjoicmF3LmdpdGh1YnVzZXJjb250ZW50LmNvbSIsImtleSI6ImtleTUiLCJleHAiOjE3MzIwMTQzNzUsIm5iZiI6MTczMjAxNDA3NSwicGF0aCI6Ii8zODY0Mjk2Ni8zODc2MjQ1MjMtMzRkYzlmOGMtNmVjZC00ZTQ0LTljYTItMTYyOWYzM2YxMmYxLnBuZz9YLUFtei1BbGdvcml0aG09QVdTNC1ITUFDLVNIQTI1NiZYLUFtei1DcmVkZW50aWFsPUFLSUFWQ09EWUxTQTUzUFFLNFpBJTJGMjAyNDExMTklMkZ1cy1lYXN0LTElMkZzMyUyRmF3czRfcmVxdWVzdCZYLUFtei1EYXRlPTIwMjQxMTE5VDExMDExNVomWC1BbXotRXhwaXJlcz0zMDAmWC1BbXotU2lnbmF0dXJlPTJlNDczMWI2OTE0ODJhMTdjNmYzMmRiNWJjN2ZhYzQxZTZjMGU0NzdmNDVkYzQ4N2M0YjE2NDY5ZGI3NTZmNDUmWC1BbXotU2lnbmVkSGVhZGVycz1ob3N0In0.xmcONO1JgocbouU4v7cpa9r1w4ftXWqmEgdTKVTWsR0)

## Further steps

1. Try parallelization for both scripts.
2. Try on-the-fly splitting instead of predetermined splitting.
