// Leetcode 120. Triangle
use std::cmp;
impl Solution {
    pub fn minimum_total(mut triangle: Vec<Vec<i32>>) -> i32 {
        let n = triangle.len();
        for i in 1..n {
            for j in 0..=i {
                let mut val: i32 = triangle[i][j];
                if(j==0){
                    val+= triangle[i-1][j];
                }else if(j==i){
                    val+= triangle[i-1][j-1];
                }else{
                    val+= cmp::min(triangle[i-1][j-1],triangle[i-1][j]);
                }
                triangle[i][j] = val;
            }
        }
        let mut min:i32 = triangle[n-1][0];
        for &i in triangle[n-1].iter() {
            min = cmp::min(min,i);
        }
        return min;
    }
}