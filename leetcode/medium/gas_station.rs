impl Solution {
    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut gasSum = 0;
        let mut costSum = 0;
        let n = cost.len();
        for i in 0..n{
            gasSum+= gas[i];
            costSum += cost[i];
        }
        if(gasSum<costSum){
            return -1;
        }
        let mut curGas : i32 =0;
        let mut start : i32 = 0;
        for i in 0..n{
            curGas += (gas[i]-cost[i]);
            if(curGas<0){
                curGas = 0;
                start = (i+1) as i32;
            }
        }
        return  start;
    }
}