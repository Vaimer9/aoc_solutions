use io_lib::Sample;

fn main() {
    let sample = Sample::new("sample.txt")
        .read()
        .unwrap()
        .get_raw().to_string();

    let input: Vec<Vec<i32>> = sample.trim_end()
        .split("\n\n")
        .map(|s| s.split('\n').map(|s| s.parse().unwrap()).collect())
        .collect();

    let mut summed: Vec<i32> = input.iter()
        .map(|s| s.iter().sum())
        .collect();
    
    summed.sort();

    let first = summed.pop().unwrap();
    let second = summed.pop().unwrap();
    let third = summed.pop().unwrap();

    println!("First answer: {}", first);
    println!("Second answer: {}", first + second + third);
}
