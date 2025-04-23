
// Leetcode 2176. Count Equal and Divisible Pairs in an Array
// https://leetcode.com/problems/count-equal-and-divisible-pairs-in-an-array/description/
// Given a 0-indexed integer array nums of length n and an integer k, return the number of pairs (i, j) such that:
// 0 <= i < j < n, nums[i] == nums[j], and (i * j) is divisible by k.
// Example 1:
// Input: nums = [3,1,2,2,2,1,3], k = 2 
// Output: 4
// Explanation: The pairs (i, j) that satisfy the conditions are:
// - (0, 6): nums[0] == nums[6] == 3 and (0 * 6) % 2 == 0
// - (1, 5): nums[1] == nums[5] == 1 and (1 * 5) % 2 == 0
// - (2, 3): nums[2] == nums[3] == 2 and (2 * 3) % 2 == 0
// - (2, 4): nums[2] == nums[4] == 2 and (2 * 4) % 2 == 0

impl Solution {
    pub fn count_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let mut count:i32 = 0;
        let n = nums.len();
        for i in 0..n{
            for j in i+1..n{
                let prod:i32 = (i*j) as i32;
                if(nums[i]==nums[j] && prod%k==0){
                    count+=1;
                }
            }
        }
        return count;
    }
}