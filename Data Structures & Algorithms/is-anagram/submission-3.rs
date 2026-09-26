// naive approach that needs, but submit it
// next submit will be an approvement of this approach
// what is the time and space complexity of this approach ?
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.clone().chars().count() != t.clone().chars().count() {
            return false;
        }
        let mut h1: HashMap<char, usize> = HashMap::new();
        let mut h2: HashMap<char, usize> = HashMap::new();

        // init hashmaps with all chars as keys, and 0 as values
        // char appear more then once == +1 to char's key's value
        for c in s.chars() {
            if !h1.contains_key(&c) {
                h1.insert(c, 0);
                continue;
            }
            let value = h1.get_mut(&c).unwrap();
            *value += 1;
            let vc = *value;
            h1.insert(c, vc);
        }

        for c in t.chars() {
            if !h2.contains_key(&c) {
                h2.insert(c, 0);
                continue;
            }
            let value = h2.get_mut(&c).unwrap();
            *value += 1;
            let vc = *value;
            h2.insert(c, vc);
        }

        if h1 == h2 {
            return true;
        }
        false
    }
}
