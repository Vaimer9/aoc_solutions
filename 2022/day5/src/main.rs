// To avoid some rather useless indexing please put the stacks in another file
// Be sure NOT to include the numbers

use io_lib::Sample;

#[derive(Debug)]
struct StackList(Vec<Vec<char>>);

#[derive(Clone, Copy)]
struct Operation {
    quantity: u32,
    src: u32,
    dest: u32
}

impl Operation {
    fn from_str(s: &str) -> Self {
        let s = s.split_whitespace()
            .enumerate()
            .filter(|(index, _)| index % 2 != 0 )
            .map(|(_, s)| s.parse().unwrap())
            .collect::<Vec<u32>>();


        Self {
            quantity: s[0],
            src: s[1],
            dest: s[2]
        }
    }
}

impl StackList {
    fn new() -> Self {
        let values = vec![
            vec!['D', 'T', 'W', 'F', 'J', 'S', 'H', 'N'],
            vec!['H', 'R', 'P', 'Q', 'T', 'N', 'B', 'G'],
            vec!['L', 'Q', 'V'],
            vec!['N', 'B', 'S', 'W', 'R', 'Q'],
            vec!['N', 'D', 'F', 'T', 'V', 'M', 'B'],
            vec!['M', 'D', 'B', 'V', 'H', 'T', 'R'],
            vec!['D', 'B', 'Q', 'J'],
            vec!['D', 'N', 'J', 'V', 'R', 'Z', 'H', 'Q'],
            vec!['B', 'N', 'H', 'M', 'S']
        ];

        Self(values)

    }

    fn crate_mover_9000(&mut self, op: Operation) {
        let len = self.0[op.src as  usize - 1].len();
        let mut rem = self.0[op.src as usize - 1].split_off(len - op.quantity as usize);
        rem.reverse(); 
        self.0[op.dest as usize - 1].extend(rem);
    }

    fn crate_mover_9001(&mut self, op: Operation) {
        let len = self.0[op.src as  usize - 1].len();
        let rem = self.0[op.src as usize - 1].split_off(len - op.quantity as usize);
        self.0[op.dest as usize - 1].extend(rem);
    }

    fn get_word(&self) -> String {
        let mut buff = String::new();

        for list in self.0.iter() {
            buff.push(*list.last().unwrap());
        }

        buff
    }
}

fn main() {
    let op_sample = Sample::new("sample.txt")
        .read().unwrap()
        .get_vec();

    let mut first = StackList::new();
    let mut second = StackList::new();

    op_sample.iter()
        .map(|s| Operation::from_str(s.as_str()))
        .for_each(|op| {
            first.crate_mover_9000(op);
            second.crate_mover_9001(op);
        });


    println!("{}", first.get_word());
    println!("{}", second.get_word());
}
