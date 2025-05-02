// Leetcode 516 Longest Palindromic Subsequence using DP

use std::cmp;
impl Solution {
    pub fn longest_palindrome_subseq(s: String) -> i32 {
        let n = s.len();
        let mut prev:Vec<i32> = vec![0;n+1];
        let r : String = Solution::reverse_string(&s);
        if(r==s){
            return n as i32;
        }
        for i in 1..=n {
            let mut cur:Vec<i32> = vec![0;n+1];
            for j in 1..=n {
                if(r.as_bytes()[i-1]==s.as_bytes()[j-1]){
                    cur[j] = prev[j-1]+1;
                }else{
                    cur[j] = cmp::max(cur[j-1],prev[j]);
                }
            }
            prev = cur;
        }
        return prev[n];
    }
    pub fn reverse_string(s:&str) -> String {
        let mut reverse : String = String::new();
        for ch in s.chars().rev() {
            reverse.push(ch);
        }
        return reverse;
    }
}