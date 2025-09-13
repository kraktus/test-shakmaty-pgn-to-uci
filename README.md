# Crude benchmark of using shakmaty to convert a specific PGN (SAN) move list to UCI

## Run the benchmark

rust: `cargo bench`
python `pytest`

## Results

On M1 MacBook Air, 16GB RAM:

rust, (two versions, one using a real PGN parser, the other a minimal adhoc one)
```
adhoc parser            time:   [6.4630 µs 6.4691 µs 6.4762 µs]
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild

pgn_reader parser       time:   [7.7191 µs 7.7251 µs 7.7309 µs]
Found 4 outliers among 100 measurements (4.00%)
  2 (2.00%) low mild
  2 (2.00%) high severe
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