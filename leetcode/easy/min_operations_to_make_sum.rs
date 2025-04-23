// LeetCode min_operations_to_make_sum.rs
// https://leetcode.com/problems/minimum-operations-to-make-the-sum-divisible-by-k/
// Given an integer array nums and an integer k, return the minimum number of operations required to make the sum of nums divisible by k.
impl Solution {
    pub fn min_operations(nums: Vec<i32>, k: i32) -> i32 {
        let mut sum:i32 = 0;
        for i in nums{
            sum+=i;
        }
        return sum%k;
    }
}