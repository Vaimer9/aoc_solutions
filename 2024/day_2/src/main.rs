use io_lib::Sample;

fn check(line: &[usize]) -> bool {
    let dir = line[1] >= line[0];
    line.windows(2)
        .all(|p| (p[1].abs_diff(p[0]) <= 3) && ((p[1] >= p[0]) == dir) && p[1] != p[0])
}

fn main() {
    let raw = Sample::new("input.txt")
        .read().unwrap()
        .get_vec();

    let lines = raw.iter()
        .map(|s| s.split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
        ).map(|line| {
            // Day 1 answer
            let f = check(&line);
            let mut s = false;

            // Day 2
            if !f {
                for i in 0..line.len() {
                    let mut t = line.clone();
                    t.remove(i);
                    if check(&t) { s = true; break; }
                }
            }

            f || s
        }).filter(|s| *s).count();

    println!("{lines:#?}");
}
