use std::cmp::max;
 use std::collections::HashSet;
pub struct SlidingWindow;

impl SlidingWindow{

    pub fn longest_substring_without_repeating_char(s: String) -> i32{

        let chars: Vec<char> = s.chars().collect();
        
        let mut seen: HashSet<char> = HashSet::new();

        let mut left = 0;
        let mut max   =0;


        for right in 0..chars.len(){

            while seen.contains(&chars[right]) {

                seen.remove(&chars[left]);
                left +=1;
                
            }

            max = max.max(right -left +1);



        }

        return max as i32;
    }

    pub fn buy_and_sell_stock(prices: Vec<i32>) ->i32 {

        let mut min_price = i32::MAX;
        let mut max_profit = 0;

        for &price in prices.iter(){

            min_price = min_price.min(price);

            max_profit = max_profit.max(price-min_price);
        }

        return max_profit;
    }
}
