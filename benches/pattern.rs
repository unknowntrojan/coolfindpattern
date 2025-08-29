#![feature(test)]

extern crate test;

use coolfindpattern::{PatternSearcher, pattern};
use rand::Rng;
use test::Bencher;

#[bench]
fn bench_pattern_1gig(b: &mut Bencher) {
    let mut rng = rand::rng();

    let size: usize = 1024 * 1024 * 1024;
    let mut test_pattern: Vec<u8> = (0..size).map(|_| rng.random_range(0..=255)).collect();

    let pattern = size - 15;

    test_pattern[pattern] = 0xDE;
    test_pattern[pattern + 1] = 0xAD;
    test_pattern[pattern + 2] = 0xFF;
    test_pattern[pattern + 3] = 0xBE;
    test_pattern[pattern + 4] = 0xEF;
    test_pattern[pattern + 5] = 0xDE;
    test_pattern[pattern + 6] = 0xAD;
    test_pattern[pattern + 7] = 0xFF;
    test_pattern[pattern + 8] = 0xBE;
    test_pattern[pattern + 9] = 0xEF;

    b.iter(|| {
        assert_eq!(
            PatternSearcher::new(
                &test_pattern,
                pattern!(0xDE, 0xAD, _, 0xBE, 0xEF, 0xDE, 0xAD, _, 0xBE, 0xEF)
            )
            .next(),
            Some(pattern)
        );
    });
}
