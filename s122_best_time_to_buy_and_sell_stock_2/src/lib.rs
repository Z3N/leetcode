impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        prices.as_slice().windows(2)
              .filter(|x| x[0] < x[1])
              .map(|prices| prices[1] - prices[0])
              .sum()
    }
}
