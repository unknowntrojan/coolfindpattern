# coolfindpattern

<a href="https://crates.io/crates/coolfindpattern"><img src="https://img.shields.io/crates/v/coolfindpattern.svg"></img></a>

an updated version of my [findpattern](https://github.com/unknowntrojan/findpattern) crate, this time using modern SIMD.

You have to enable SIMD instruction sets in your .cargo/config.toml. See [config.toml](./.cargo/config.toml)

## Old Version (naive)

![Old Benchmark (285ms/GB)](./images/old.png)

## New Version (SSE2)

![New Benchmark SSE2 (21ms/GB)](./images/sse2.png)

## New Version (AVX2)

![New Benchmark AVX2 (17ms/GB)](./images/avx2.png)
