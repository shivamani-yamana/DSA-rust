use std::cmp;
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        let n = height.len();
        let mut left:Vec<i32> = vec![0;n];
        let mut right:Vec<i32> = vec![0;n];
        let mut leftMax : i32 = 0;
        let mut rightMax : i32 = 0;
        for i in 0..n{
            left[i] = cmp::max(leftMax,height[i]);
            leftMax = left[i];
            right[n-1-i] = cmp::max(rightMax,height[n-1-i]);
            rightMax = right[n-1-i];
        }
        let mut res:i32 = 0;
        for i in 0..n {
            let s = cmp::min(left[i],right[i]);
            res+=(s-height[i]);
        }
        res
    }
}