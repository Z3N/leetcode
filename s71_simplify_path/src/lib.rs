impl Solution {
    pub fn simplify_path(path: String) -> String {
        let mut result = Vec::new();
        for word in path.split('/') {
            match word.as_bytes() {
                [b'.', b'.'] => {
                    result.pop();
                }
                [b'.'] => {}
                [] => {}
                _ => result.push(word.to_string())
            }
        }
        format!("/{}", result.join("/"))
    }
}
struct Solution;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::simplify_path("/home//foo/".to_string());
        assert_eq!(result, "/home/foo".to_string());
    }
    #[test]
    fn it_does_not_work() {
        let result = Solution::simplify_path("/a/./b/../../c/".to_string());
        assert_eq!(result, "/c".to_string());
    }
    #[test]
    fn it_does_not_work_too() {
        let result = Solution::simplify_path("/a/../../b/../c//.//".to_string());
        assert_eq!(result, "/c".to_string());
    }
}
