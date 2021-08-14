pub struct Solution {}
// ! https://leetcode.com/problems/race-car/discuss/1388604/Rust-DP-and-Python-BFS
use std::cmp::min;
impl Solution {
    pub fn racecar(target: i32) -> i32 {
        // dp[i]: min instructions to move car from pos 0 to i, with
        //        initial speed 1 towards destination

        let n: usize = target as usize;
        let mut dp: Vec<i32> = vec![std::i32::MAX; n + 1];
        dp[0] = 0; // base case

        for i in 1..n + 1 {
            let power = (i as f64 + 1.0).log2();
            if power.fract() == 0.0 {
                // if it can be reached by pure acceleration
                // where i == 2 ** n - 1
                dp[i] = power as i32;
            } else {
                // 2 ** (n-1) - 1 < i < 2 ** n - 1, where n is the steps of acceleration
                // accelerating [step] times to reach the lower bound
                let mut step: i32 = power.floor() as i32;
                // position after acceleration, equivalent to 2 ** (n-1) - 1
                let mut j: usize = (1 << step) - 1;
                // accelerating backwards [bw] times
                for bw in 0..step {
                    // distance that is gone backwards
                    let distance: usize = (1 << bw) - 1;
                    dp[i] = min(dp[i], step + 1 + bw + 1 + dp[i - (j - distance)])
                }
                // step over one more time to reach the upper bound
                step += 1;
                j = (1 << step) - 1;
                dp[i] = min(dp[i], step + 1 + dp[j - i])
            }
        }

        dp[n]
    }
}
