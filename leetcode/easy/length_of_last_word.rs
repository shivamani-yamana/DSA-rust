impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut length : i32 = 0;
        let mut isWord : bool = false;
        for i in s.chars().rev() {
            if(i==' ' && isWord){
                return length;
            }
            if(i!=' '){
                isWord = true;
                length+=1;
            }
        }
        return length;
    }
}