#![feature(test)]

extern crate test;

use coolfindpattern::{pattern, PatternSearcher};
use rand::Rng;
use test::Bencher;

#[bench]
fn bench_pattern_1gig(b: &mut Bencher) {
    let mut rng = rand::thread_rng();

    // let size: usize = 1024 * 1024 * 1024;
    let size: usize = 268435456;
    let mut test_pattern: Vec<u8> = (0..size).map(|_| rng.gen_range(0..=255)).collect();

    // let pattern = size / 2;
    let pattern = size - 5;

    test_pattern[pattern] = 0xDE;
    test_pattern[pattern + 1] = 0xAD;
    test_pattern[pattern + 2] = 0xFF;
    test_pattern[pattern + 3] = 0xBE;
    test_pattern[pattern + 4] = 0xEF;

    b.iter(|| {
        assert_eq!(
            PatternSearcher::new(&test_pattern, pattern!(0xDE, 0xAD, _, 0xBE, 0xEF)).next(),
            Some(pattern)
        );
    });
}
