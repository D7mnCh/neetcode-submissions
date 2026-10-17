/*
 - solution didn't work -.-, just get the solution...
 - the problem with current solution is when the both array's elements solutions are eq
 - and also the solution of the above solution is based O(N^2)
*/
impl Solution {
    // returned indices is based "nums"
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // need to map nums elements with there indices
        let mut sorted_nums = nums.clone();
        sorted_nums.sort();

        // NOTE can't have duplicate keys so handle it here ?
        // if nums[i] == nums[j] && nums[i] + nums[j] == target {
        //     return vec![i,j];
        // }

        // used to map value from "sorted_nums" to an index from "nums"
        let mut unsorted_nums_hash_maped: HashMap<&i32, usize> = HashMap::new();
        for (i, num) in nums.iter().enumerate() {
            dbg!((i, num));

            // can't use a hash map to store item-index,
            //if nums[i] + nums[j] == target where nums[i] == nums[j], -> i think i am wrong
            //a hash-map cannot have duplicate key
            unsorted_nums_hash_maped.insert(num, i);
            // [4,3,4,7], target = 8
            // [4:0 , 3:1 , 4:2 , 7:3] -> can't have duplicate keys
        }

        // NOTE a shavel happen, but why/how ?
        dbg!(&unsorted_nums_hash_maped);

        let mut i = 0;
        let mut j = nums.len() - 1;

        while i != j {
            // found values
            if sorted_nums[i] + sorted_nums[j] == target {
                return vec![
                    unsorted_nums_hash_maped[&sorted_nums[i]] as i32,
                    unsorted_nums_hash_maped[&sorted_nums[j]] as i32,
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
