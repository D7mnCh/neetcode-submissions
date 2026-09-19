impl Solution {
    pub fn has_duplicate(mut nums: Vec<i32>) -> bool {
        /*
         if you start from 0, you will hit out of bound cuz you will use
         nums[i + 1], and as you can see if i is the last index, i + 1 is out
         of bound
        */
        for i in 1..nums.len() {
            nums.sort();
            if nums[i] == nums[i - 1] {
                return true;
            }
        }
        false
    }
}

