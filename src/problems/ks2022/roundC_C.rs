use std::io::{StdinLock, StdoutLock};

type Console<'a> = IO<StdinLock<'a>, StdoutLock<'a>>;
pub struct IO<R, W: std::io::Write>(R, std::io::BufWriter<W>);

impl<R: std::io::Read, W: std::io::Write> IO<R, W> {
    pub fn new(r: R, w: W) -> IO<R, W> {
        IO(r, std::io::BufWriter::new(w))
    }
    pub fn write<S: ToString>(&mut self, s: S) {
        use std::io::Write;
        self.1.write_all(s.to_string().as_bytes()).unwrap();
    }
    pub fn read<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        let buf = self
            .0
            .by_ref()
            .bytes()
            .map(|b| b.unwrap())
            .skip_while(|&b| b == b' ' || b == b'\n' || b == b'\r' || b == b'\t')
            .take_while(|&b| b != b' ' && b != b'\n' && b != b'\r' && b != b'\t')
            .collect::<Vec<_>>();
        unsafe { std::str::from_utf8_unchecked(&buf) }
            .parse()
            .ok()
            .expect("Parse error.")
    }
    pub fn usize0(&mut self) -> usize {
        self.read::<usize>() - 1
    }
    pub fn vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.read()).collect()
    }
    pub fn chars(&mut self) -> Vec<char> {
        self.read::<String>().chars().collect()
    }

    pub fn binary_vec(&mut self) -> Vec<u8> {
        self.read::<String>().bytes().map(|b| b - b'0').collect()
    }
}

struct Solution {}

impl Solution {
    pub fn solve(console: &mut Console) -> String {
        // get nums of ants and stick length
        let ants_nums: usize = console.read();
        let stick_len: usize = console.read::<usize>();

        // get ants info: <label, position, direction>
        let mut ants: Vec<(usize, usize, usize)> = Vec::new();
        for i in 0..ants_nums {
            ants.push((i + 1, console.read::<usize>(), console.read::<usize>()));
        }

        // calc distance to edge: (distance, orientation)
        let mut events: Vec<(usize, usize)> = Vec::new();
        for ant in ants.iter() {
            if ant.2 == 0 {
                events.push((ant.1, 0));
            } else {
                events.push((stick_len - ant.1, 1));
            }
        }

        ants.sort_by(|a, b| a.1.cmp(&b.1)); // sort by position
        events.sort();

        // get order with label: (distance, label)
        let mut order: Vec<(usize, usize)> = Vec::new();
        let (mut l, mut r) = (0, ants_nums - 1);
        for event in events.iter() {
            if event.1 == 0 {
                // pop left value
                order.push((event.0, ants[l].0));
                l += 1;
            } else {
                // pop last value
                order.push((event.0, ants[r].0));
                r -= 1;
            }
        }

        // sort ordering with label
        order.sort();
        order
            .into_iter()
            .map(|(dist, label)| label.to_string())
            .collect::<Vec<_>>()
            .join(" ")
    }
}

fn main() {
    let (r, w) = (std::io::stdin(), std::io::stdout());
    let mut console: Console = IO::new(r.lock(), w.lock());

    let nums: usize = console.read();

    for i in 0..nums {
        let result = Solution::solve(&mut console);
        console.write(format!(
            "Case #{idx}: {value}\n",
            idx = i + 1,
            value = result
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn case1() {
        main();
    }
}
