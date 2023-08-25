# coolfindpattern

<a href="https://crates.io/crates/coolfindpattern"><img src="https://img.shields.io/crates/v/coolfindpattern.svg"></img></a>

an updated version of my [findpattern](https://github.com/unknowntrojan/findpattern) crate, this time using modern SIMD.

You have to enable SIMD instruction sets in your [.cargo/config.toml](./.cargo/config.toml).

Benchmarks were performed on the following machine:

- AMD Ryzen 7 5800X3D boosting to ~4.3GHz
- 4*16 GB DDR4-3200 Dual-Channel CL16
- (Other specs don't matter)

**! The old measurements were incorrect/misleading! I forgot to change back the memory size back to 1GiB from 250MiB. !**

## Average Time taken (needle=10byte, haystack=1GiB)

| | Os | O1 | O2 | O3 |
| --- | --- | --- | --- | --- |
| Old Impl | 1000ms | 4008ms | 391.7ms | 273.1ms
| New SSE2 | 92.3ms | 297.7ms | 81.5ms | 94.6ms
| New AVX2 | 64.3ms | 191ms | 66ms | 78.5ms

## Average Speed in GiB/s (needle=10byte, haystack=1GiB)

| | Os | O1 | O2 | O3 |
| --- | --- | --- | --- | --- |
| Old Impl | 0.98GiB/s | 0.249GiB/s | 2.55GiB/s | 3.6GiB/s
| New SSE2 | 10.8GiB/s | 3.36GiB/s | 12.34GiB/s | 10.57GiB/s
| New AVX2 | 15.5GiB/s | 5.23GiB/s | 15.1GiB/s | 12.7GiB/s
