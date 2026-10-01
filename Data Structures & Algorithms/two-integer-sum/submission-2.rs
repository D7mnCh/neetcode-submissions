/*
    - Brute force method: iter through the array with two nested loops,
    using indices i and j , where j start from i + 1

    - if num[i] + num[j] == target && i != j, return an array of i and j
    where the first index is the smallest one
*/

impl Solution {
    // Vec<i32> should be Vec<usize>, i am return indecies not the elements
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                if nums[i] + nums[j] == target {
                    return vec![i as i32, j as i32];
                }
            }
        }

        Vec::new()
    }
}
