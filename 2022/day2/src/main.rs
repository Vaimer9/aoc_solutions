use io_lib::Sample;

#[derive(Copy, Clone, Debug)]
enum Answer {
    Rock,
    Paper,
    Scissors
}

impl From<&str> for Answer {
    fn from(s: &str) -> Self {
        match s {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => todo!()
        }
    }
}

impl From<Answer> for i32 {
    fn from(a: Answer) -> Self {
        match a {
            Answer::Rock => 1,
            Answer::Paper => 2,
            Answer::Scissors => 3
        }
    }
}

// Cue the operator abuse
impl std::ops::Add for Answer {
    type Output = i32;

    fn add(self, rhs: Self) -> Self::Output {
        use Answer::*;

        match (self, rhs) {
            // Win
            (Scissors, Rock)
            | (Paper, Scissors)
            | (Rock, Paper) => 6 + i32::from(rhs),

            // Lose
            (Rock, Scissors)
            | (Scissors, Paper)
            | (Paper, Rock) => i32::from(rhs),

            // Draw
            (Rock, Rock)
            | (Paper, Paper)
            | (Scissors, Scissors) => 3 + i32::from(rhs),
        }
    }
}

// Rock = Lose
// Paper = Draw
// Scissors = Win

impl std::ops::Mul for Answer {
    type Output = i32;

    fn mul(self, rhs: Self) -> Self::Output {
        use Answer::*;
        
        match (self, rhs) {
            (lhs, Rock) => i32::from(-(-lhs)),
            (lhs, Paper) => 3 + i32::from(lhs),
            (lhs, Scissors) => 6 + i32::from(-lhs)
        }
    }
}

impl std::ops::Neg for Answer {
    type Output = Self;

    fn neg(self) -> Self::Output {
        use Answer::*;

        match self {
            Rock => Paper,
            Paper => Scissors,
            Scissors => Rock,
        }
    }
}

fn main() {
    let sample = Sample::new("sample.txt")
        .read()
        .unwrap()
        .get_raw()
        .to_owned();

    let input_iter = sample.trim_end()
        .split('\n')
        .map(|xs| xs.split_whitespace().map(Answer::from).collect::<Vec<Answer>>());


    let first_answer: i32 = input_iter.clone()
        .map(|xs| xs[0] + xs[1]).sum();

    let second_answer: i32 = input_iter
        .map(|xs| xs[0] * xs[1]).sum();


    println!("First answer: {first_answer}");
    println!("Second answer: {second_answer}");
}
