use std::{io::{StdinLock, StdoutLock}, fmt::format};

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
    fn check_valid(N: usize, X: usize, Y: usize) -> Option<usize> {
        let sum_N = (N+1)*N/2;
        if sum_N%(X+Y)==0 {
            Some(sum_N/(X+Y))
        } else {
            None
        }
    }

    pub fn solve(console: &mut Console) -> String {
        let nxy: Vec<usize> = console.vec(3);
        if let Some(ratio) = Solution::check_valid(nxy[0], nxy[1], nxy[2]) {
            let mut aim = nxy[1]*ratio;
            let mut buf = Vec::new();
            for i in (1..=nxy[0]).rev() {
                if aim>=i {
                    aim -= i;
                    buf.push(i);
                }
            }
            if aim==0 {
                buf.reverse();
                let mut res_set = String::new();
                for x in &buf {
                    if res_set.len()>0 {res_set.push(' ')};
                    res_set.push_str(&x.to_string());
                }
                format!("POSSIBLE\n{}\n{}", buf.len(), res_set)
            } else {
                "IMPOSSIBLE".to_string()
            }
        } else {
            "IMPOSSIBLE".to_string()
        }
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
