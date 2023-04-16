use rand::prelude::SliceRandom;
use rand::thread_rng;

fn main() {
    let range = (1..51).collect::<Vec<u32>>();
    println!("{:?}", range);
    let count = range.len() / 3;
    let ids = build(range, count);
    println!("{:?}", ids);
    let solution = resolve(ids);
    println!("{:?}", solution);
}

fn resolve(mut v: Vec<u32>) -> Vec<(u32, u32)> {
    v.sort();
    let mut result = Vec::with_capacity(v.len());
    let mut it = v.into_iter();

    if let Some(mut left) = it.next() {
        let mut expected = left + 1;
        while let Some(next) = it.next() {
            if expected == next {
                expected += 1;
            } else {
                result.push((left, expected-1));
                left = next;
                expected = left + 1;
            }
        }
        result.push((left, expected-1));
    }

    return result;
}

fn build(mut v: Vec<u32>, n: usize) -> Vec<u32> {
    v.shuffle(&mut thread_rng());
    v[0..n].sort();
    v[0..n].to_vec()
}
