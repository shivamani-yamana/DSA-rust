use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map : HashMap<i32,i32> = HashMap::new();
        for i in 0..nums.len() {
            let remaining = target - nums[i];
            if let Some(&index) = map.get(&remaining) {
                return vec![index,i as i32];
            }
            map.insert(nums[i],i as i32);
        }
        return vec![];
    }
}