use io_lib::Sample;

struct Range {
    upper: u32,
    lower: u32,
}

impl From<&str> for Range {
    fn from(s: &str) -> Self {
        let (lower, upper) = s.split_once('-').unwrap();
        let lower = lower.parse::<u32>().unwrap();
        let upper = upper.parse::<u32>().unwrap();

        Self { lower, upper }
    }
}

impl Range {
    fn contains(&self, rhs: &Self) -> bool {
        rhs.lower >= self.lower && rhs.upper <= self.upper
    }

    fn overlaps(&self, rhs: &Self) -> bool {
        // x1 <= y2 && y1 <= x2
        self.lower <= rhs.upper && rhs.lower <= self.upper
    }
}

fn main() {
    let sample = Sample::new("sample.txt")
        .read().unwrap()
        .get_vec();

    let answer_template = sample.iter()
        .map(|s| s.split_once(',').unwrap())
        .map(|s| (Range::from(s.0), Range::from(s.1)));

    let first_answer = answer_template.clone()
        .filter(|(first, second)| first.contains(second) || second.contains(first))
        .count();

    let second_answer = answer_template
        .filter(|(first, second)| first.overlaps(second))
        .count();

    println!("{first_answer}");
    println!("{second_answer}");
}
