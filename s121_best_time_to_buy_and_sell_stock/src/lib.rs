impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min = prices[0];
        prices.into_iter()
              .map(|price| {
                  let result = price - min;
                  min = min.min(price);
                  result
              })
              .collect::<Vec<i32>>()
              .into_iter()
              .max()
              .unwrap_or(0)
    }
    pub fn max_profit_fold(prices: Vec<i32>) -> i32 {
        prices.into_iter()
              .fold((i32::MAX, 0), |(min, max), price|
                  (min.min(price),
                   max.max(price - min))
              )
              .1
    }
}

pub struct Solution;