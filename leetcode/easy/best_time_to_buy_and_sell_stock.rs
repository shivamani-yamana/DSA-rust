use std::cmp;
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut minP = prices[0];
        let mut maxP = 0;
        for price in prices.iter(){
            maxP = cmp::max(maxP,price-minP);
            minP = cmp::min(minP,*price);
        }
        return maxP;
    }
}