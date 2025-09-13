# Crude benchmark of using shakmaty to convert a specific PGN (SAN) move list to UCI

## Run the benchmark

rust: `cargo bench`
python `pytest`

## Results

On M1 MacBook Air, 16GB RAM:

rust
```
 Running benches/my_benchmark.rs (/.cargo/target/release/deps/my_benchmark-c458262784c1d66d)
Gnuplot not found, using plotters backend
go                      time:   [6.3047 µs 6.3531 µs 6.4229 µs]
                        change: [+0.4095% +1.9866% +4.2506%] (p = 0.03 < 0.05)
                        Change within noise threshold.
Found 13 outliers among 100 measurements (13.00%)
  5 (5.00%) high mild
  8 (8.00%) high seve
```

python
```
------------------------------------------------------- benchmark: 1 tests -------------------------------------------------------
Name (time in us)                  Min       Max      Mean   StdDev    Median      IQR  Outliers  OPS (Kops/s)  Rounds  Iterations
----------------------------------------------------------------------------------------------------------------------------------
test_benchmark_str_to_uci     829.4580  878.7500  843.8982  10.0198  839.7910  17.3125     249;0        1.1850     719           1
----------------------------------------------------------------------------------------------------------------------------------

Legend:
  Outliers: 1 Standard Deviation from Mean; 1.5 IQR (InterQuartile Range) from 1st Quartile and 3rd Quartile.
  OPS: Operations Per Second, computed as 1 / Mean

```