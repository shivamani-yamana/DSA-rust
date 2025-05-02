// Leetcode 1143 Longest Common Subsequence using DP
use std::cmp;
impl Solution {
    pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
        let n = text1.len();
        let m = text2.len();
        let mut prev : Vec<i32> = vec![0;m+1];
        for i in 1..=n {
            let mut cur : Vec<i32> = vec![0;m+1];
            for j in 1..=m {
                if(text1.as_bytes()[i-1] == text2.as_bytes()[j-1]){
                    cur[j] = prev[j-1]+1;
                }else{
                    cur[j] = cmp::max(cur[j-1],prev[j]);
                }
            }
            prev = cur;
        }
        return prev[m];
    }
}