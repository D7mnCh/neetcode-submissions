// hash set length method, we know that a hashset only contain
//unique elements so if a dublicate, it will not get inserted 
//into the set which means it should be less then nums length
// this method used as alternative to the Hashset with condition
// to reduce just loc :)
use std::collections::HashSet;
impl Solution {
    pub fn has_duplicate(nums: Vec<i32>) -> bool {
        let mut set: HashSet<_> = nums.iter().copied().collect();
        set.len() < nums.len()
    }
}