impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if(x<0){
            return false;
        }
        else if(x==0){
            return true;
        }
        else{
            let mut reverse : i32 = 0;
            let mut temp = x;
            while(temp>0){
                let rem = temp%10;
                reverse = reverse*10 + rem;
                temp = temp/10;
            }
            print!("{}",reverse);
            return (reverse == x);
        }
    }
}