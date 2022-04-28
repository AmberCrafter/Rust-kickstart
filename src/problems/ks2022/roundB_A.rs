struct Solution {}

impl Solution {
    pub fn infinity_area(r: i32, a: i32, b: i32) -> f64 {
        let mut area = 0.0;
        let mut r = r;
        let pi: f64 = 1.0_f64.atan() * 4.0;

        let mut flag_a = true;
        while r>0 {
            area += pi * (r * r) as f64;
            if flag_a {
                r*=a;
            } else {
                r/=b;
            }
            flag_a = !flag_a;
        }
        (area*1e6).round()*1e-6
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1() {
        let inputs = (1,3,6);
        let expect= 31.415927;
        let result = Solution::infinity_area(inputs.0,inputs.1,inputs.2);

        println!("Result: {result:?}");
        assert_eq!(expect, result);
    }

    #[test]
    fn case2() {
        let inputs = (5,2,5);
        let expect= 455.530935;
        let result = Solution::infinity_area(inputs.0,inputs.1,inputs.2);

        println!("Result: {result:?}");
        assert_eq!(expect, result);
    }

}