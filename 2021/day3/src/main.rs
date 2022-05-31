use io_lib::Sample;

fn main() {
    let sample = Sample::new("sample.txt")
        .read().unwrap();

    let vec = Vec::from(&sample);
    println!("{:-^20}", "Part 1");

    part1(vec.clone());

    println!("{:-^20}", "\nPart 2");
    let oxygen = isize::from_str_radix(
        &part2(vec.clone(), 0, Choice::Oxygen).to_string(),
        2
    ).unwrap();

    let co2 = isize::from_str_radix(
        &part2(vec, 0, Choice::CO2).to_string(),
        2
    ).unwrap();

    println!("Oxygen level: {oxygen}\nCO2 level: {co2}");
    println!("Answer: {}", oxygen * co2);
}


enum Choice {
    Oxygen,
    CO2
}

fn part2(input: Vec<String>, index: usize, choice: Choice) -> usize {
    let mut one = (Vec::new(), 0);
    let mut zero = (Vec::new(), 0);

    if input.len() == 1 {
        return input[0].parse().unwrap();
    }

    for y in input.iter() {
        match y.chars().nth(index).unwrap() {
            '1' => {
                one.0.push(y.clone());
                one.1 += 1;
            },
            '0' => {
                zero.0.push(y.clone());
                zero.1 += 1;
            },
            _ => ()
        }
    }

    match choice {
        Choice::Oxygen => {
            match sort(one.1, zero.1) {
                (1, 0) => part2(one.0, index + 1, choice),
                (0, 1) => part2(zero.0, index + 1, choice),
                _ => 0
            }
        }
        Choice::CO2 => {
            match sort(one.1, zero.1) {
                (1, 0) => part2(zero.0, index + 1, choice),
                (0, 1) => part2(one.0, index + 1, choice),
                _ => 0
            }
        }
    }
}

fn part1(input: Vec<String>) {
    let mut gamma = Vec::new();
    let mut epsilon = Vec::new();

    // Till the end of the first line
    for x in 0..input[0].len() {
        let mut one: usize = 0;
        let mut zero: usize = 0;
        
        // For ever line
        for y in input.iter() {
            match y.chars().nth(x).unwrap() {
                '1' => one += 1,
                '0' => zero += 1,
                _   => (),
            }
        }

        let (greater, smaller) = sort(one, zero);

        gamma.push(greater);
        epsilon.push(smaller);
    }

    let gamma_num = to_str(&gamma);
    let epsilon_num = to_str(&epsilon);

    println!("Gamma: {:?} -> {}", gamma, gamma_num);
    println!("Epsilon: {:?} -> {}", epsilon, epsilon_num);

    println!("Answer = {}", isize::from_str_radix(&gamma_num, 2).unwrap() * isize::from_str_radix(&epsilon_num, 2).unwrap());

}

#[inline]
fn sort(ones: usize, zeroes: usize) -> (u8, u8) {
    if ones >= zeroes {
        (1, 0)
    } else {
        (0, 1)
    }
}

#[inline]
fn to_str(input: &[u8]) -> String {
    input.iter()
        .map(|s| char::from_digit(*s as u32, 10).unwrap())
        .collect()
}
