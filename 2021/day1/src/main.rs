use io_lib::Sample;

fn main() {
    let sample = Sample::new("sample.txt")
        .read()
        .unwrap();

    let input = Vec::from(&sample)
        .iter()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    part1(input.clone());
    part2(input);
}

fn part1(input: Vec<i32>) {
    let mut windows = input[..].windows(2);
    let mut count = 0;

    for _ in 0..windows.len() {
        if let Some(&[a, b]) = windows.next() {
            if b > a { count += 1; }
        } else {
            break;
        }
    }

    println!("Part 1 Answer: {count}");
}

fn part2(input: Vec<i32>) {
    let mut windows = input[..].windows(3);
    let mut count = 0;
    let mut prevsum = 0;

    for _ in 0..windows.len() {

        if let Some(&[a, b, c]) = windows.next() {
            let sum = a + b + c;

            // Don't add on first check
            if prevsum == 0 { prevsum = sum; continue; }
            if sum > prevsum { count += 1; }
            prevsum = sum;
        } else {
            break;
        }
    }

    println!("Part 2 Answer: {count}");
}
