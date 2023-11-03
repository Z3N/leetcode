impl Solution {
    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut result = Vec::new();
        for token in tokens {
            match token.as_str() {
                "+" => {
                    let rhs = result.pop().unwrap();
                    let lhs = result.pop().unwrap();
                    result.push(lhs + rhs);
                }
                "*" => {
                    let rhs = result.pop().unwrap();
                    let lhs = result.pop().unwrap();
                    result.push(lhs * rhs);
                }
                "-" => {
                    let rhs = result.pop().unwrap();
                    let lhs = result.pop().unwrap();
                    result.push(lhs - rhs);
                }
                "/" => {
                    let rhs = result.pop().unwrap();
                    let lhs = result.pop().unwrap();
                    result.push(lhs / rhs);
                }
                _ => result.push(token.parse::<i32>().unwrap())
            }
        }
        result.pop().unwrap()
    }
}
struct Solution;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::eval_rpn(vec!["4".to_string(),
                                             "13".to_string(),
                                             "5".to_string(),
                                             "/".to_string(),
                                             "+".to_string(),]);
        assert_eq!(result, 6);
    }
    #[test]
    fn it_works_too() {
        let result = Solution::eval_rpn(vec!["10".to_string(),
                                             "6".to_string(),
                                             "9".to_string(),
                                             "3".to_string(),
                                             "+".to_string(),
                                             "-11".to_string(),
                                             "*".to_string(),
                                             "/".to_string(),
                                             "*".to_string(),
                                             "17".to_string(),
                                             "+".to_string(),
                                             "5".to_string(),
                                             "+".to_string()]);
        assert_eq!(result, 22);
    }
}
