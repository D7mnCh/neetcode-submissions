// solution didn't work -.-
impl Solution {
    // returned indices is based on the orignal nums,
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // need to map nums elements with there indices
        let mut sorted_nums = nums.clone();
        sorted_nums.sort();

        let mut unsorted_nums_map_item_index = HashMap::new();
        for (i, n) in nums.iter().enumerate() {
            // can't use a hash map to store item-index, if nums[i] + nums[j] == target where, nums[i] == nums[j],
            //a hash-map cannot have duplicate key
            unsorted_nums_map_item_index.insert(n, i);
        }

        let mut i = 0;
        let mut j = nums.len() - 1;

        while i != j {
            if sorted_nums[i] + sorted_nums[j] == target {
                return vec![
                    unsorted_nums_map_item_index[&nums[i]] as i32,
                    unsorted_nums_map_item_index[&nums[j]] as i32,
                ];
            } else if sorted_nums[i] + sorted_nums[j] < target {
                i += 1;
            } else {
                j -= 1;
            }
        }

        return Vec::new();
    }
}

