pub mod configuration;

pub(crate) use rand::prelude::*;

fn main() {
    let cfg = configuration::read("configuration.toml");

    let mut input: Vec<u32> = {
        let rng = rand::thread_rng();
        let dist = rand::distributions::Uniform::new_inclusive(cfg.input.min, cfg.input.max);
        rng.sample_iter(dist)
            .take(cfg.input.count as usize)
            .collect()
    };

    let mut values_a = input.clone();
    let mut values_b: Vec<u32> = std::iter::repeat(0).take(input.len()).collect();

    let mut buckets_a: Vec<(usize, usize)> = vec![(0, input.len())];
    let mut buckets_b = Vec::new();

    radix_sort_iter(7, &values_a[..], buckets_a.drain(..), &mut values_b[..], &mut buckets_b);
    radix_sort_iter(6, &values_b[..], buckets_b.drain(..), &mut values_a[..], &mut buckets_a);
    radix_sort_iter(5, &values_a[..], buckets_a.drain(..), &mut values_b[..], &mut buckets_b);
    radix_sort_iter(4, &values_b[..], buckets_b.drain(..), &mut values_a[..], &mut buckets_a);
    radix_sort_iter(3, &values_a[..], buckets_a.drain(..), &mut values_b[..], &mut buckets_b);
    radix_sort_iter(2, &values_b[..], buckets_b.drain(..), &mut values_a[..], &mut buckets_a);
    radix_sort_iter(1, &values_a[..], buckets_a.drain(..), &mut values_b[..], &mut buckets_b);
    radix_sort_iter(0, &values_b[..], buckets_b.drain(..), &mut values_a[..], &mut buckets_a);

    let expected = {
        input.sort();
        input
    };
    let actual = values_a;

    assert_eq!(expected, actual);
}

const RADIX: u32 = 4;
const RADIX_MASK: u32 = (1 << RADIX) - 1;

fn compute_bin(round: u32, value: u32) -> u32 {
    (value >> (round * RADIX)) & RADIX_MASK
}

fn radix_sort_iter(round: u32, values: &[u32], buckets: impl Iterator<Item = (usize, usize)>, new_values: &mut[u32], new_buckets: &mut Vec<(usize, usize)>) {
    assert_eq!(0, new_buckets.len());

    for bucket in buckets {
        // Compute histogram.
        let lengths = values[bucket.0 .. bucket.1].iter().fold([0; 1 << RADIX], |mut lengths, &val| {
            let bin = compute_bin(round, val) as usize;
            lengths[bin] += 1;
            lengths
        });

        // Prefix sum histogram.
        let offsets = {
            let mut offsets = [0; 1 << RADIX];
            let mut acc = 0;
            for i in 1..16 {
                acc += lengths[i - 1];
                offsets[i] = acc;
            }
            offsets
        };

        let mut counters = [0; 1 << RADIX];

        for i in 0..(bucket.1 - bucket.0) {
            let val = values[bucket.0 + i];
            let bin = compute_bin(round, val) as usize;

            let o = (offsets[bin] + counters[bin]) as usize;
            counters[bin] += 1;

            new_values[bucket.0 + o] = val;
        }

        assert_eq!(&lengths[..], &counters[..]);

        for i in 0..(1 << RADIX) {
            match lengths[i] {
                0 => {
                    // No need to sort 0 elements.
                },
                1 | _ => {
                    // No need to sort 1 element, but it still needs to be copied to the other buffer.
                    let begin = bucket.0 + offsets[i] as usize;
                    let end = begin + lengths[i] as usize;
                    new_buckets.push((begin, end));
                }
            }
        }
    }
}
