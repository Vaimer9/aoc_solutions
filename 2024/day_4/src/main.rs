use io_lib::Sample;

#[derive(PartialEq, Eq)]
enum Pos {
    Up,
    Le,
    Ri,
    Do,
}

fn part1(data: &[Vec<char>], coord: (usize, usize)) -> usize {
    let mut possible = vec![Pos::Up, Pos::Do, Pos::Ri, Pos::Le];
    let mut count = 0;

    match coord {
        (0..3, 0..3) => { possible.remove(0); possible.remove(3 - 1); }, // Offset by one because element just removed
        (0..3, 137..140) => { possible.remove(0); possible.remove(2 - 1); },
        (137..140, 0..3) => { possible.remove(1); possible.remove(3 - 1); },
        (137..140, 137..140) => { possible.remove(1); possible.remove(2 - 1); },
        (0..3, _) => { possible.remove(0); },
        (_, 0..3) => { possible.remove(3); },
        (137..140, _) => { possible.remove(1); },
        (_, 137..140) => { possible.remove(2); },
        (_, _) => ()
    }

    if  possible.contains(&Pos::Ri) {
        if data[coord.0][coord.1 + 1] == 'M'
        && data[coord.0][coord.1 + 2] == 'A'
        && data[coord.0][coord.1 + 3] == 'S' { count += 1; }
    }

    if possible.contains(&Pos::Le) {
        if data[coord.0][coord.1 - 1] == 'M'
        && data[coord.0][coord.1 - 2] == 'A'
        && data[coord.0][coord.1 - 3] == 'S' { count += 1; }
    }

    if possible.contains(&Pos::Up) {
        if data[coord.0 - 1][coord.1] == 'M'
        && data[coord.0 - 2][coord.1] == 'A'
        && data[coord.0 - 3][coord.1] == 'S' { count += 1; }
    }

    if possible.contains(&Pos::Do) {
        if data[coord.0 + 1][coord.1] == 'M'
        && data[coord.0 + 2][coord.1] == 'A'
        && data[coord.0 + 3][coord.1] == 'S' { count += 1; }
    }

    if possible.contains(&Pos::Up) && possible.contains(&Pos::Ri) {
        if data[coord.0 - 1][coord.1 + 1] == 'M'
        && data[coord.0 - 2][coord.1 + 2] == 'A'
        && data[coord.0 - 3][coord.1 + 3] == 'S' { count += 1; }
    }

    if possible.contains(&Pos::Up) && possible.contains(&Pos::Le) {
        if data[coord.0 - 1][coord.1 - 1] == 'M'
        && data[coord.0 - 2][coord.1 - 2] == 'A'
        && data[coord.0 - 3][coord.1 - 3] == 'S' { count += 1; }
    }

    if possible.contains(&Pos::Do) && possible.contains(&Pos::Le) {
        if data[coord.0 + 1][coord.1 - 1] == 'M'
        && data[coord.0 + 2][coord.1 - 2] == 'A'
        && data[coord.0 + 3][coord.1 - 3] == 'S' { count += 1; }
    }

    if possible.contains(&Pos::Do) && possible.contains(&Pos::Ri) {
        if data[coord.0 + 1][coord.1 + 1] == 'M'
        && data[coord.0 + 2][coord.1 + 2] == 'A'
        && data[coord.0 + 3][coord.1 + 3] == 'S' { count += 1; }
    }

    return count;
}

fn part2(data: &[Vec<char>], coord: (usize, usize)) -> bool {
    let mut possible = true;

    match coord {
        (0, _) | (_, 0) | (139, _) | (_, 139) => { possible = false; }
        (_, _) => ()
    }

    if possible {
        if ((data[coord.0 - 1][coord.1 - 1] == 'M' && data[coord.0 + 1][coord.1 + 1] == 'S')
            || (data[coord.0 - 1][coord.1 - 1] == 'S' && data[coord.0 + 1][coord.1 + 1] == 'M'))
        && ((data[coord.0 - 1][coord.1 + 1] == 'M' && data[coord.0 + 1][coord.1 - 1] == 'S')
            || (data[coord.0 - 1][coord.1 + 1] == 'S' && data[coord.0 + 1][coord.1 - 1] == 'M'))
        {
            return true;
        }
    }

    return false;
}

fn main() {
    let sample = Sample::new("input.txt")
        .read().unwrap();

    let data = sample.get_vec().iter()
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut p1_count = 0;
    let mut p2_count = 0;

    for i in 0..data.len() {
        for j in 0..data[i].len() {
            match data[i][j] {
                'X' => { p1_count += part1(&data, (i, j)); },
                'A' => { if part2(&data, (i, j)) { p2_count += 1; } }
                _ => ()
            }
        }
    }

    println!("Part 1: {p1_count}");
    println!("Part 2: {p2_count}");
}
