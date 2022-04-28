struct Solution {}

impl Solution {
    fn reverse_number(value: i32) -> i32 {
        let mut value = value;
        let mut res = 0;
        while value>0 {
            res *= 10;
            res = value%10;
            value /= 10;
        }
        res
    }

    pub fn palindromic_factor(value: i32) -> i32 {
        let mut factor_count = 0;
        let mut factor = 1;
        while factor<value/factor {
            if value%factor==0 {
                if factor == Solution::reverse_number(factor) {factor_count+=1}
                if value/factor == Solution::reverse_number(value/factor) {factor_count+=1}
            }
            factor += 1;
        }

        factor_count
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let inputs = 6;
        let expect= 4;
        let result = Solution::palindromic_factor(inputs);

        println!("Result: {result:?}");
        assert_eq!(expect, result);
    }

    #[test]
    fn case2() {
        let inputs = 10;
        let expect= 3;
        let result = Solution::palindromic_factor(inputs);

        println!("Result: {result:?}");
        assert_eq!(expect, result);
    }

    #[test]
    fn case3() {
        let inputs = 144;
        let expect= 7;
        let result = Solution::palindromic_factor(inputs);

        println!("Result: {result:?}");
        assert_eq!(expect, result);
    }
}