use io_lib::Sample;
use std::{collections::HashSet, slice::Windows};

fn unique(iter: &[char]) -> bool {
    let mut uniq = HashSet::new();
    iter.iter().all(move |x| uniq.insert(x))
}

fn main() {
    let sample = Sample::new("sample.txt")
        .read().unwrap()
        .get_raw()
        .chars()
        .collect::<Vec<char>>();

    let find_num = |xs: Windows<char>| xs.map(unique)
        .enumerate()
        .find(|(_, x)| *x)
        .unwrap()
        .0;

    let ans1 = find_num(sample.windows(4));
    let ans2 = find_num(sample.windows(14));

    println!("{:#?}", ans1 + 4);
    println!("{:#?}", ans2 + 14)
}
