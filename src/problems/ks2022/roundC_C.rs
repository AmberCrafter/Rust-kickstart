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
    fn check_destination(ants: &Vec<(usize, usize, usize)>, dest: usize, orientation: usize) -> bool {
        ants.iter().any(|x| x.1==dest && x.2==orientation)
    }
    fn drop_ant(recorder: &mut String, ants: &mut Vec<(usize, usize, usize)>, stick_len: usize) {
        for i in (0..ants.len()).rev() {
            if ants[i].1<=0 || ants[i].1>=stick_len {
                if recorder.len()>0 {recorder.push(' ')}
                recorder.push_str(&ants[i].0.to_string());
                ants.remove(i);
            }
        }
    }
    pub fn solve(console: &mut Console) -> String {
        // get nums of ants and stick length
        let ants_nums: usize = console.read();
        let stick_len: usize = console.read::<usize>()*2;

        // get ants info: <label, position, direction>
        let mut ants: Vec<(usize, usize, usize)> = Vec::new();
        for i in 0..ants_nums {
            ants.push((i+1, console.read::<usize>()*2, console.read::<usize>()));
        }

        // reverse ants ordering to make result ordering correct!
        ants.reverse();

        // calculate result
        let mut res = String::new();
        Solution::drop_ant(&mut res, &mut ants, stick_len);
        while ants.len()>0 {
            let ants_clone = ants.clone();
            for ant in ants.iter_mut() {
                match ant.2 {
                    0 => {
                        if ant.1>=2 && Solution::check_destination(&ants_clone, ant.1-2, 1) {
                            ant.1-=1;
                            ant.2=1;
                        } else {
                            ant.1-=1;
                        }
                    },
                    1 => {
                        if ant.1<=(stick_len-2) && Solution::check_destination(&ants_clone, ant.1+2, 0) {
                            ant.1+=1;
                            ant.2=0;
                        } else {
                            ant.1+=1;
                        }
                    },
                    _ => {}
                }
            }
            Solution::drop_ant(&mut res, &mut ants, stick_len);
        }
        res
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