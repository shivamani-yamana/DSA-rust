// LeetCode Problem 14
// https://leetcode.com/problems/longest-common-prefix/
// Given a list of strings, find the longest common prefix string among them. If there is no common prefix, return an empty string "".
// Example 1:
// Input: strs = ["flower","flow","flight"]
// Output: "fl"
// Example 2:
// Input: strs = ["dog","racecar","car"]
// Output: ""
// Explanation: There is no common prefix among the input strings.

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut prevString  = strs[0].as_str();
        for i in 1..strs.len() {
            prevString = Solution::common_prefix(prevString,&strs[i]);
            if (prevString.is_empty()){
                break;
            }
        }
        return prevString.to_string();
    }
    pub fn common_prefix<'a>(s1: &'a str, s2: &'a str) -> &'a str{
        let mut i : usize = 0;
        while (i<s1.len() && i<s2.len() && s1.as_bytes()[i] == s2.as_bytes()[i]){
            i+=1;
        }
        return &s1[0..i];
    }
}