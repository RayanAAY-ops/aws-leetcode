use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        
        let mut hashmap = HashMap::new();
        let n = nums.len();

        for i in 0..n {
            let mut complement = target - nums[i];

            if let Some(&index) = hashmap.get(&complement) {
                return vec![i as i32, index];
            }

            hashmap.insert(nums[i], i as i32);    
        }

        vec![]
    }
}





// Calling get:

//     hashmap.get(&complement) returns an Option<&i32> because get gives a reference to the value, not a copy of it.
//     The Option type is a Rust enum that can either be Some(&value) if the key exists or None if the key does not exist.

// Pattern Matching with if let Some(&index):

//     We use if let Some(&index) to safely extract the value from the Option.
//     Some(&index) means we're getting the reference &i32 from the Option.
//     The & before index is needed because get gives us a reference (&i32), but we want the value itself (i32), so we dereference it to get i32 (not a reference).


// Accessing HashMap values: Instead of indexing with hashmap[complement] like Python, you should use hashmap.get(&complement) since get returns an Option<&V>