use io_lib::Sample;


fn common_char((first, second): (&str, &str)) -> char {
    first.chars()
        .find(|c| second.contains(*c))
        .unwrap()
}

fn common_char_p2((s1, s2, s3): (&str, &str, &str)) -> char {
    
    // I hate this hack, im sorry
    let (second, third);

    let smallest = if s1 <= s2 {
        if s1 <= s3 {
            second = s3;
            third = s2;
            s1
        } else {
            second = s1;
            third = s2;
            s3
        }
    } else  if s2 <= s3 {
        second = s1;
        third = s3;
        s2
    } else {
        second = s2;
        third = s1;
        s3
    };
    
    smallest.chars()
        .find(|c| second.contains(*c) && third.contains(*c))
        .unwrap()
}

fn main() {
    let sample = Sample::new("sample.txt")
        .read().unwrap()
        .get_vec();

    // println!("{sample:?}");

    let first_answer: i32 = sample.iter()
        .map(|s| s.split_at(s.len() / 2))
        .map(|xs| i32::from(Char(common_char(xs))))
        .sum();

    let second_answer: i32 = sample.chunks(3)
        .map(|s| (&*s[0], &*s[1], &*s[2]))
        .map(|xs| i32::from(Char(common_char_p2(xs))))
        .sum();

    println!("First answer {first_answer}");
    println!("Second answer {second_answer}");
}

struct Char(char);

impl From<Char> for i32 {
    fn from(Char(c): Char) -> Self {
        if c.is_lowercase() {
            (c as u8 - 0x60).into()
        } else {
            i32::from(Char(c.to_ascii_lowercase())) + 26
        }
    }
}
