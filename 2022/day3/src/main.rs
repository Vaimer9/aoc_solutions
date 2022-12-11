use std::fs::read_to_string;

fn main() {
    let sample = read_to_string("sample.txt")
        .expect("Missing Sample")
        .trim_end()
        .lines()
        .map(|line| line.chars().map(|ch| ch.to_priority()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    assert!(sample.len() > 0);

    let first_answer = part_1(&sample);
    let second_answer = part_2(&sample);

    println!("First answer {first_answer}");
    println!("Second answer {second_answer}");
}

fn part_1(input: &[Vec<u8>]) -> u16 {
    input
        .iter()
        .map(|line| get_common_item(line).expect("Missing common item for part 1!"))
        .sum()
}

fn get_common_item(v: &[u8]) -> Option<u16> {
    let (a, b) = v.split_at(v.len() / 2);
    let common = a.iter().find(|p| b.contains(*p))?;
    return Some(*common as u16);
}

fn part_2(input: &[Vec<u8>]) -> u16 {
    input.chunks(3).fold(0 as u16, |acc, v| {
        if let [a, b, c] = v {
            if let Some(common) = a.iter().find(|it| b.contains(it) && c.contains(it)) {
                return *common as u16 + acc;
            } else {
                panic!("No common item found")
            }
        } else {
            panic!("Invalid slice shape")
        }
    })
}

trait ChExtension {
    fn to_priority(self) -> u8;
}

impl ChExtension for char {
    fn to_priority(self) -> u8 {
        if self.is_lowercase() {
            (self as u8) - b'a' + 1
        } else {
            (self as u8) - b'A' + 27
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{get_common_item, part_1, part_2, ChExtension};

    const INPUT: &str = r#"vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"#;

    #[test]
    fn test_parsing() {
        let sample = INPUT
            .lines()
            .map(|l| -> Vec<_> { l.chars().map(|ch| ch.to_priority()).collect() })
            .collect::<Vec<_>>();

        assert!(sample.len() > 0);

        for item in sample.iter() {
            println!("{:?}", get_common_item(item));
        }

        assert_eq!(get_common_item(&sample[0]), Some(16u16));
        assert_eq!(get_common_item(&sample[sample.len() - 1]), Some(19u16));
        assert_eq!(part_1(&sample), 157 as u16);
        assert_eq!(part_2(&sample), 70 as u16);
    }
}
