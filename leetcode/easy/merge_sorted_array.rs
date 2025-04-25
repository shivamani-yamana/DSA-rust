impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut i = (m+n-1);
        let mut n1 = m-1;
        let mut n2 = n-1;
        while(n1>=0 && n2>=0){
            if(nums1[n1 as usize]>=nums2[n2 as usize]){
                nums1[i as usize] = nums1[n1 as usize];
                n1-=1;
            }else{
                nums1[i as usize] = nums2[n2 as usize];
                n2-=1;
            }
            i-=1;
        }
        while(n1>=0){
            nums1[i as usize] = nums1[n1 as usize];
            n1-=1;
            i-=1;
        }
        while(n2>=0){
            nums1[i as usize] = nums2[n2 as usize];
            n2-=1;
            i-=1;
        }
    }
}