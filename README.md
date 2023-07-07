# coolfindpattern

<a href="https://crates.io/crates/coolfindpattern"><img src="https://img.shields.io/crates/v/coolfindpattern.svg"></img></a>

an updated version of my [findpattern](https://github.com/unknowntrojan/findpattern) crate, this time using modern SIMD.

You have to enable SIMD instruction sets in your [.cargo/config.toml](./.cargo/config.toml).

Benchmarks were performed on the following machine:

- AMD Ryzen 7 5800X3D boosting to ~4.3GHz
- 4*16 GB DDR4-3200 Dual-Channel CL16
- (Other specs don't matter)

**! The old measurements were incorrect/misleading! I forgot to change back the memory size back to 1GB from 250MB. !**

## Average Time taken (needle=10byte, haystack=1GB)

| | Os | O1 | O2 | O3 |
| --- | --- | --- | --- | --- |
| Old Impl | 1000ms | 4008ms | 391.7ms | 273.1ms
| New SSE2 | 92.3ms | 297.7ms | 81.5ms | 94.6ms
| New AVX2 | 64.3ms | 191ms | 66ms | 78.5ms

## Average Speed in GB/s (needle=10byte, haystack=1GB)

| | Os | O1 | O2 | O3 |
| --- | --- | --- | --- | --- |
| Old Impl | 0.98GB/s | 0.249GB/s | 2.55GB/s | 3.6GB/s
| New SSE2 | 10.8GB/s | 3.36GB/s | 12.34GB/s | 10.57GB/s
| New AVX2 | 15.5GB/s | 5.23GB/s | 15.1GB/s | 12.7GB/s
