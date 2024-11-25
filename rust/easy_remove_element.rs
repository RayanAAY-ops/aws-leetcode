impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut index :usize = 0;

        for i in 0..nums.len(){
            if nums[i] != val {
                nums[index] = nums[i];
                index +=1;
            }
        }
    
    index as i32
    }
}

// The loop does not create an immutable reference to nums as a whole.
// Instead, nums[i] gives direct access to elements, and the mutable borrow (nums[index] = nums[i]) is // //well-scoped and doesn't overlap with the loop itself.