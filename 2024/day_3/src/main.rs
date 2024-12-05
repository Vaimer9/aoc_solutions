use regex::Regex;
use io_lib::Sample;

fn main() {
    let data = Sample::new("input.txt")
        .read().unwrap();

    println!("{}", part1(data.get_raw()));
    println!("{}", part2(data.get_raw()));
}

fn part1(raw: &str) -> usize {
    let re = Regex::new(r"mul\((?<f>[0-9]*),(?<s>[0-9]*)\)").unwrap();

    re.captures_iter(raw)
        .map(|s| ((&s["f"]).parse::<usize>().unwrap(), (&s["s"]).parse::<usize>().unwrap()))
        .map(|(f, s)| f * s).sum()
}

fn part2(raw: &str) -> usize {
    let re = Regex::new(r"(mul\((?<f>[0-9]*),(?<s>[0-9]*)\))|(?<do>do\(\))|(?<dont>don\'t\(\))").unwrap();

    let captures = re.captures_iter(raw).into_iter().collect::<Vec<_>>();

    let mut count = 0;
    let mut switch = true;

    for x in captures {
        match (x.name("f"), x.name("s")) {
            (Some(f), Some(s)) => if switch { count += f.as_str().parse::<usize>().unwrap() * s.as_str().parse::<usize>().unwrap() }
            _ => ()
        }

        if let Some(_) = x.name("do") { switch = true; }
        if let Some(_) = x.name("dont") { switch = false; }
    }

    return count;
}
