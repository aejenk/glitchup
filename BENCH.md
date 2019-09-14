Benchmark settings:

```toml
times = 1
iterations = [1000]
chunksize = [1000]
mutations = [
    Chaos * 4,
    Void * 4,
    Increase * 4,
    Gradient * 4
]

times = 1

iterations = [1000]
chunksize = [1000]

mutations = [
    ["Chaos", "Chaos", "Chaos", "Chaos"],
    ["Void" , "Void" , "Void" , "Void"],
    ["Increase","Increase","Increase","Increase"],
    ["Gradient","Gradient","Gradient","Gradient"]
]

[IncreaseConfig]
increase_by = [1,255]

[GradientConfig]
accelerate_by = [1,2]
accelerate_in = [100_000, 5_000_000]
```

**serial:**

```
Time (mean ± σ):     807.1 ms ±  38.6 ms    [User: 2.7 ms, System: 6.8 ms]
Range (min … max):   744.3 ms … 869.1 ms    10 runs
```

**parallel:**

```
Time (mean ± σ):     517.3 ms ±  37.2 ms    [User: 1.4 ms, System: 8.5 ms]
Range (min … max):   467.8 ms … 570.3 ms    10 runs
```

*The parallel approach takes 64% of the time* ...at the cost of the code looking like hot garbage.