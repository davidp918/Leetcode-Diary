fn main() {
    // playground
    println!("");
    println!("{}", Solution::longest_valid_parentheses("()".to_string()));
}
// ? https://leetcode.com/problems/longest-valid-parentheses/discuss/1415072/Rust-DP

struct Solution {}
impl Solution {
    pub fn longest_valid_parentheses(s: String) -> i32 {
        // dp[i]: longest valid parentheses that ends with s[i]
        // dp[i] = max(2 + dp[i-2] if s[i-1] == '(' and s[i] == ')',
        //             2 + dp[i-1] + dp[i - 1 - dp[i-1] - 1]
        //             if s[i - 1 - dp[i-1]] == '(' and s[i] == ')' and s[i-1] == ')'
        //            )
        let n: usize = s.len();
        let s = s.into_bytes();
        let mut dp: Vec<i32> = vec![0; n];

        for i in 1..n {
            if s[i] == b'(' {
                continue;
            }
            // then bytes[i] must be ')'
            // if the last two char is '()'
            if s[i - 1] == b'(' {
                dp[i] = 2 + {
                    // checked access to dp
                    if i >= 2 {
                        dp[i - 2]
                    } else {
                        0
                    }
                }
            // if the char before 「the valid parentheses which end with the second last ')'」 is '('
            } else if s.get(i - 1 - dp[i - 1] as usize) == Some(&b'(') {
                // chained with the valid parentheses before
                dp[i] = 2 + dp[i - 1] + dp.get(i - 1 - dp[i - 1] as usize - 1).unwrap_or(&0);
            }
        }

        dp.into_iter().max().unwrap_or(0)
    }
}
