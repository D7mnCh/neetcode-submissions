// brute force
// space complexity is O(n log n + m log m) cuz i am sorting two strings
//(inputs)
// sapce compelxity i think it's constant
use itertools::Itertools;
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        // using itertools to sort the strings
        // let s = s.chars().sorted();
        // let t = t.chars().sorted();
        let mut s: Vec<_> = s.chars().collect::<Vec<char>>();
        let mut t: Vec<_> = t.chars().collect::<Vec<char>>();
        s.sort();
        t.sort();

        if s == t {
            return true;
        }
        false
    }
}
