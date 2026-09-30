/*
using an array ? or a hash table

 1. return if s and t differ in length
 2.create an array of 26 (english letter) to iter through it
 each english letter from the array has a corresponding inedex
 3.increment the array at the index corresponding to s[i]
 4.decrement the array at the index corresponding to t[i]
 5.should the array be zeroed at the end
 
 space compelxity O(1), but it's more effient then hashtable approach
 cuz no hashing
 
*/
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.chars().count() != t.chars().count() {
            return false;
        }
        let mut array = [0; 26];
        // a must equal to b in order this iter work as intended
        for (a, b) in s.as_bytes().into_iter().zip(t.as_bytes()) {
            // (x-y), where 'x' is the smallest byte's numeric value which is 'a'
            //and 'y' could be the the biggest 'z', that range from 'a' to 'z' is
            // 0..26
            array[(a - b'a') as usize] += 1;
            array[(b - b'a') as usize] -= 1;
        }
        dbg!(&array);
        array.into_iter().all(|x| x == 0)
    }
}