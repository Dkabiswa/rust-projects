use std::collections::HashMap;

fn main() {
    let v = vec![2, 4, 6, 8, 12, 24, 12, 11];
    let median = find_mode(&v);
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

fn find_mode(v: &[u32]) -> u32 {
    let len = v.len();
    if len == 0 {
        panic!("Cannot compute mode of an empty vector");
    }
    let mut tracking = HashMap::new();
    let mut mode: u32 = 0;
    let mut max: u32 = 0;

    for &item in v {
        let count = tracking.entry(item).or_insert(0);
        *count += 1;
        if *count > max {
            max = *count;
            mode = item;
        }
    }
    mode
}

/*
Convert strings to pig latin. The first consonant of each word is moved to the
end of the word and ay is added, so first becomes irst-fay.
Words that start with a vowel have hay added to the end instead (apple becomes apple-hay).
Keep in mind the details about UTF-8 encoding!
*/

/*
Using a hash map and vectors, create a text interface to allow a user to add employee names
to a department in a company; for example, “Add Sally to Engineering” or “Add Amir to Sales.”
Then let the user retrieve a list of all people in a department or all people in the company by department,
sorted alphabetically.

*/
