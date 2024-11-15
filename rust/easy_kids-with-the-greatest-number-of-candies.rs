impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        let mut res:Vec<bool> = Vec::with_capacity(candies.len());
        let candies_max = candies.iter().max().copied().unwrap_or(0);
        
        for (i, candy) in candies.iter().enumerate() {
            res.push(candy + extra_candies >= candies_max);
        }
    
    res

    }
}