use std::collections::{HashMap, HashSet};

#[cfg(test)]

mod test {

    use super::*;

    #[test]

    fn tests_hashmap() {
        let person1 = "alice";
        let person2 = "bob";

        let mut results_hm: HashMap<&str, u32> = HashMap::new();

        results_hm.insert("person1", 55);
        results_hm.insert("person2", 60);

        let test_score = results_hm.get("person1");
        dbg!(test_score);
    }

    #[test]
    fn tests_hashset() {
        let mut name_hs: HashSet<&str> = HashSet::new();

        name_hs.insert("alice");
        name_hs.insert("bob");
        name_hs.insert("jane");

        if name_hs.contains("bob") {
            dbg!("bob is in the set!");
        }
    }
}
