use io_lib::Sample;

fn main() {
    let sample = Sample::new("sample.txt")
        .read().unwrap();

    let input = Vec::from(&sample);

    println!("{:->10}", "Part 1");
    part1(input.clone());

    println!("\n{:->10}", "Part 2");
    part2(input);
}

fn part1(input: Vec<String>) {
    let mut horizontal = 0;
    let mut depth= 0;

    for x in input.iter() {

        let y = x.split(' ').collect::<Vec<&str>>();

        match y[0] {
            "forward" => horizontal += y[1].parse::<i32>().unwrap(),
            "up" => depth -= y[1].parse::<i32>().unwrap(),
            "down" => depth += y[1].parse::<i32>().unwrap(),
            _ => ()
        }
    }

    println!("Horizontal: {horizontal}");
    println!("Depth: {depth}");
    println!("Product: {}", horizontal * depth);
}

fn part2(input: Vec<String>) {
    let mut depth = 0;
    let mut horizontal = 0;
    let mut aim = 0;

    for x in input.iter() {

        let y = x.split(' ').collect::<Vec<&str>>();

        match y[0] {
            "forward" => {
                let num = y[1].parse::<i32>().unwrap();
                horizontal += num;
                depth += num * aim;

            },
            "up" => aim -= y[1].parse::<i32>().unwrap(),
            "down" => aim += y[1].parse::<i32>().unwrap(),
            _ => ()
        }
    }

    println!("Aim: {aim}\nHorizontal: {horizontal}\nDepth: {depth}");
    println!("Product: {}", horizontal * depth );
}
