// next submit will be an approvement of this approach
// what is the time and space complexity of this approach ?
// i wanna review on the comments
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        // if they differ in length, just return
        if s.clone().chars().count() != t.clone().chars().count() {
            return false;
        }

        // construct
        let mut h1: HashMap<char, usize> = HashMap::new();
        let mut h2: HashMap<char, usize> = HashMap::new();

        // insert key with 0, if key found +1 to that key's value
        // NOTE entry method allow in place minipulation after inserting a key
        for (s_char, t_char) in s.chars().zip(t.chars()) {
            *h1.entry(s_char).or_insert(0) += 1;
            *h2.entry(t_char).or_insert(0) += 1;
        }

        if h1 == h2 {
            return true;
        }
        false
    }
}

