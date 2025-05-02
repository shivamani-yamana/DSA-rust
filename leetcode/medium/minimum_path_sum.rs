// Leetcode 64. Minimum Path Sum
use std::cmp;
impl Solution {
    pub fn min_path_sum(mut grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let m = grid[0].len();
        for i in 1..n {
            grid[i][0] += grid[i-1][0];
        }

        for j in 1..m {
            grid[0][j] += grid[0][j-1];
        }

        for i in 1..n {
            for j in 1..m {
                grid[i][j] += cmp::min(grid[i-1][j],grid[i][j-1]);
            }
        }

        return grid[n-1][m-1];
    }
}