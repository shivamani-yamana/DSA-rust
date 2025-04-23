use std::collections::HashMap;
impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut romans : HashMap<char,i32> = HashMap::new();
        romans.insert('I',1);
        romans.insert('V',5);
        romans.insert('X',10);
        romans.insert('L',50);
        romans.insert('C',100);
        romans.insert('D',500);
        romans.insert('M',1000);

        let mut result : i32 = 0;
        let mut prevVal : i32 = 0;
        for ch in s.chars().rev(){
            if let Some(&curVal) = romans.get(&ch) {
                if(curVal >= prevVal){
                    result += curVal;
                }else{
                    result -= curVal;
                }
                prevVal = curVal;
            }
        }

        return result;
    }
}