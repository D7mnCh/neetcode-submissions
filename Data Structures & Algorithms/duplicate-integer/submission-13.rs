// hash set method
use std::collections::HashSet;
impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut new = HashSet::new();
        for i in 0..nums.len() {
            if !new.insert(nums[i]) {
                return true;
            }
        }
        false
    }
}