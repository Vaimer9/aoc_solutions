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

fn main() {
    let sample = Sample::new("sample.txt")
        .read()
        .unwrap()
        .get_raw()
        .to_string();

    let answer: i32 = sample.trim_end()
        .split('\n')
        .map(|xs| xs.split_whitespace().map(Answer::from).collect::<Vec<Answer>>())
        .map(|xs| xs[0] + xs[1])
        .sum();

    println!("{answer}");
}
