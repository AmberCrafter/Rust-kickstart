struct Solution {}

impl Solution {
    pub fn unlock_the_padlock(locks: Vec<usize>, range: usize) -> usize {
        let mut res = usize::MAX;
        let mut buf = Vec::new();

        let mut locks = locks;
        
        while locks.len()>0 {
            let mut step = 0;
            let mut idx_buf = 0;
            let mut idx_locks = 0;
            let mut current = locks[0];

            while (idx_locks<locks.len() || idx_buf<buf.len()) {
                let buffer_step = if buf.len()>idx_buf {
                    let buf_value = buf[buf.len()-1-idx_buf];
                    Some(current.abs_diff(buf_value).min(range-current+buf_value))
                } else {
                    None
                };

                let locks_step = if locks.len()>idx_locks {
                    let lock_value = locks[idx_locks];
                    Some(current.abs_diff(lock_value).min(range-current+lock_value))
                } else {
                    None
                };

                if buffer_step.unwrap_or(usize::MAX)<locks_step.unwrap_or(usize::MAX) {
                    current = buf[buf.len()-1-idx_buf];
                    step += buffer_step.unwrap();
                    idx_buf+=1;
                } else {
                    current = locks[idx_locks];
                    step += locks_step.unwrap();
                    idx_locks+=1;
                }
            }

            res = res.min(step+current.min(current.abs_diff(range)));
            buf.push(locks[0]);
            locks = locks[1..].to_vec();
        }
        res
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let inputs = (vec![1,1,0,1,0,1], 2);
        let expect= 3;
        let result = Solution::unlock_the_padlock(inputs.0, inputs.1);

        println!("Result: {result:?}");
        assert_eq!(expect, result);
    }

    #[test]
    fn case2() {
        let inputs = (vec![0,1,0,0,1,1], 2);
        let expect= 2;
        let result = Solution::unlock_the_padlock(inputs.0, inputs.1);

        println!("Result: {result:?}");
        assert_eq!(expect, result);
    }

    #[test]
    fn case3() {
        let inputs = (vec![1,1,2,2,3,3], 10);
        let expect= 3;
        let result = Solution::unlock_the_padlock(inputs.0, inputs.1);

        println!("Result: {result:?}");
        assert_eq!(expect, result);
    }

    #[test]
    fn case4() {
        let inputs = (vec![1,1,9,9,1,1], 10);
        let expect= 3;
        let result = Solution::unlock_the_padlock(inputs.0, inputs.1);

        println!("Result: {result:?}");
        assert_eq!(expect, result);
    }
}