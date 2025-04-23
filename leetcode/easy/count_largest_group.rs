// Leetcode 1399. Count Largest Group
// https://leetcode.com/problems/count-largest-group/description/
// Given an integer n, you have to count the number of groups of integers in the range [1, n] such that the sum of the digits of each integer in the group is equal to the sum of the digits of all integers in that group.

use std::collections::HashMap;
impl Solution {
    pub fn count_largest_group(n: i32) -> i32 {
        let map: HashMap<i32,i32> = Solution::createGroups(n);
        let mut count : i32 = 0;
        let mut size : i32 = 0;
        for (key,val) in map.into_iter() {
            if(val>size){
                size = val;
                count = 1;
            }else if(val==size){
                count+=1;
            }
        }
        return count;
    }

    pub fn getSum(mut n:i32)-> i32 {
        let mut sum:i32 = 0;
        while(n>0){
            sum += n%10;
            n/=10;
        }
        return sum;
    }

    pub fn createGroups(n:i32) -> HashMap<i32,i32> {
        let mut map: HashMap<i32,i32> = HashMap::new();
        for i in 1..=n {
            let sum:i32 = Solution::getSum(i);
            if(!map.contains_key(&sum)){
                map.insert(sum,1);
            }else{
                let val:i32 = map.get(&sum).copied().unwrap_or(0);
                map.insert(sum,val+1);
            }
        }
        return map;
    }
}