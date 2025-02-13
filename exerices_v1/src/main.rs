use std::collections::HashMap;

fn main() {
    let median = find_median(vec![2, 4, 6, 8, 12, 24, 13, 11]);
    println!("The median is {median}");
}

fn find_median(mut v: Vec<i32>) -> f64 {
    let len = v.len();
    if len == 0 {
        panic!("Cannot compute median of an empty vector");
    }

    v.sort();

    if len % 2 == 1 {
        v[len / 2] as f64
    } else {
        (v[len / 2 - 1] as f64 + v[len / 2] as f64) / 2.0
    }
}

fn find_mode(&mut v: Vec<u32>) -> u32 {
    let mut tracking = HashMap::new();
}
