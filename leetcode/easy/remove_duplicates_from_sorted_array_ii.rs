impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut res:i32 = 0;
        for i in 0..nums.len(){
            let index:usize = res as usize;
            if(index<2 || nums[i]!=nums[index-2]){
                nums[index] = nums[i];
                res+=1;
            }
        }
        return res;
    }
}