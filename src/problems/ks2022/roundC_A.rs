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
    fn has_uppercase(s: &mut String) {
        for c in s.chars().into_iter() {
            match c {
                'A'..='Z' => {return},
                _ => {}
            }
        }
        s.push('A')
    }

    fn has_lowercase(s: &mut String) {
        for c in s.chars().into_iter() {
            match c {
                'a'..='z' => {return},
                _ => {}
            }
        }
        s.push('a')
    }

    fn has_digit(s: &mut String) {
        for c in s.chars().into_iter() {
            match c {
                '0'..='9' => {return},
                _ => {}
            }
        }
        s.push('0')
    }

    fn has_spacial_char(s: &mut String) {
        for c in s.chars().into_iter() {
            match c {
                '#' | '@' | '*' | '&' => {return},
                _ => {}
            }
        }
        s.push('&')
    }

    fn has_enough_chars(s: &mut String, num: usize) {
        while s.len()<num {
            s.push('0')
        }
    }

    pub fn solve(console: &mut Console) -> String {
        // get nums of password
        let nums: usize = console.read();
        // get password
        let mut password: String = console.read();

        Solution::has_uppercase(&mut password);
        Solution::has_lowercase(&mut password);
        Solution::has_digit(&mut password);
        Solution::has_spacial_char(&mut password);
        Solution::has_enough_chars(&mut password,7);
        password
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
