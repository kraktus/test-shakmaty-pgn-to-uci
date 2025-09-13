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
---------------------------------------------------------------------------------------- benchmark: 2 tests ---------------------------------------------------------------------------------------
Name (time in us)               Min                 Max                Mean            StdDev              Median               IQR            Outliers  OPS (Kops/s)            Rounds  Iterations
---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
test_pyo3                  119.2080 (1.0)      163.9590 (1.0)      122.0303 (1.0)      2.2685 (1.0)      121.6250 (1.0)      1.1240 (1.0)       376;424        8.1947 (1.0)        5440           1
test_bench_pure_python     823.2499 (6.91)     936.8750 (5.71)     834.4701 (6.84)     9.8897 (4.36)     832.8750 (6.85)     5.7082 (5.08)        44;38        1.1984 (0.15)        705           1
---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
```