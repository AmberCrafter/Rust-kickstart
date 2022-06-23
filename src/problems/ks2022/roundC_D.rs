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
    fn is_palindromic(s: &Vec<char>) -> usize {
        for i in 0..s.len()/2 {
            if s[i]!=s[s.len()-1-i] {return 0;}
        }
        1
    }

    fn next(total: &mut usize, sum: usize, s: Vec<char>){
        if s.len()>0 {
            for i in 0..s.len() {
                let mut s_clone = s.clone();
                s_clone.remove(i);
                Solution::next(total, sum+Solution::is_palindromic(&s_clone), s_clone);
            }
        } else {
            *total += sum;
        }
    }

    fn factorial(n: usize) -> usize {
        if n>1 {
            n*Solution::factorial(n-1)
        } else {
            n
        }
    }

    fn calc_modulo(numerator: usize, denominator: usize) -> String {
        let M = 1000000007;
        for i in 1..denominator {
            let numer =M*i+numerator;
            if numer%denominator==0 {
                return (numer/denominator).to_string();
            }
        }
        "".to_string()
    }

    pub fn solve(console: &mut Console) -> String {
        let nums_chars: usize = console.read();
        let inputs: Vec<char> = console.chars();
        let denominator = Solution::factorial(nums_chars);
        let mut numerator = 0;
        Solution::next(&mut numerator, 0, inputs);
        // println!("{numerator}/{denominator}");
        if numerator%denominator == 0 {
            (numerator/denominator).to_string()
        } else {
            Solution::calc_modulo(numerator, denominator)
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
