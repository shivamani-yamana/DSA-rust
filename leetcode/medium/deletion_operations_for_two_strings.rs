// Leetcode 583
use std::cmp;
impl Solution {
    pub fn min_distance(word1: String, word2: String) -> i32 {
        let n = word1.len();
        let m = word2.len();
        let mut prev : Vec<i32> = vec![0;m+1];
        for i in 1..=n {
            let mut cur : Vec<i32> = vec![0;m+1];
            for j in 1..=m {
                if(word1.as_bytes()[i-1] == word2.as_bytes()[j-1]) {
                    cur[j] = prev[j-1]+1;
                }else{
                    cur[j] = cmp::max(cur[j-1],prev[j]);
                }
            }
            prev = cur;
        }
        let res1 = (n+m) as i32; //To make usize to i32
        return res1-2*(prev[m]) as i32; //FOrmula to find min operations
    }
}