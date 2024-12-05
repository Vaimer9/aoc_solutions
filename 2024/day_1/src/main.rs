use io_lib::Sample;

fn main() {
    let mut left = Vec::new();
    let mut right = Vec::new();

    Sample::new("input.txt")
        .read().unwrap()
        .get_vec()
        .iter()
        .map(|s| s.split_whitespace().collect::<Vec<_>>())
        .for_each(|s| {
            left.push(s[0].parse::<usize>().unwrap());
            right.push(s[1].parse::<usize>().unwrap());
        });

    left.sort();
    right.sort();

    let day1 = left.iter()
        .zip(right.iter())
        .fold(0, |acc, s| acc + s.0.abs_diff(*s.1));

    let day2 = left.iter()
        .map(|s| *s * right.iter().filter(|e| **e == *s).count())
        .reduce(|acc, s| acc + s)
        .unwrap();

    println!("Day 1: {day1}\nDay 2: {day2}");
}
