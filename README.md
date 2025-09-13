# Crude benchmark of using shakmaty to convert a specific PGN (SAN) move list to UCI

## Run

`cargo bench`

## Results

On M1 MacBook Air, 16GB RAM:

```
 Running benches/my_benchmark.rs (~/.cargo/target/release/deps/my_benchmark-c458262784c1d66d)
Gnuplot not found, using plotters backend
go                      time:   [6.3047 µs 6.3531 µs 6.4229 µs]
                        change: [+0.4095% +1.9866% +4.2506%] (p = 0.03 < 0.05)
                        Change within noise threshold.
Found 13 outliers among 100 measurements (13.00%)
  5 (5.00%) high mild
  8 (8.00%) high seve
```