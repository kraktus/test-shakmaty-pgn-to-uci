# Crude benchmark of using shakmaty to convert a specific PGN (SAN) move list to UCI

## Run the benchmark

rust: `cargo bench`

python: 
```
python3 -m venv venv --upgrade-deps && \
source venv/bin/activate  && \
pip3 install maturin pytest pytest-benchmark chess  && \
cd san_to_uci && maturin develop && \
pytest
```

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

python, pure python, and two wrappers to rust calling 
```
---------------------------------------------------------------------------------------- benchmark: 3 tests ----------------------------------------------------------------------------------------
Name (time in us)               Min                 Max                Mean            StdDev              Median                IQR            Outliers  OPS (Kops/s)            Rounds  Iterations
----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
test_pyo3                  119.9170 (1.0)      153.9590 (1.0)      122.3301 (1.0)      1.2607 (1.02)     122.2080 (1.0)       1.0000 (1.0)       573;184        8.1746 (1.0)        5385           1
test_pyo3_visitor          152.1670 (1.27)     182.2921 (1.18)     154.8061 (1.27)     1.2310 (1.0)      154.7500 (1.27)      1.0000 (1.0)        520;81        6.4597 (0.79)       5341           1
test_bench_pure_python     819.2080 (6.83)     885.7080 (5.75)     833.5228 (6.81)     9.2806 (7.54)     829.1671 (6.78)     17.0942 (17.09)       276;1        1.1997 (0.15)        815           1
----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
```