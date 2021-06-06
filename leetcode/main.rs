impl Solution {
    pub fn count_matches(items: Vec<Vec<String>>, rule_key: String, rule_value: String) -> i32 {
        let i = match &rule_key[..] {
            "type" => 0,
            "color" => 1,
            "name" => 2,
            _ => 0,
        };
        let mut count = 0;
        for item in items {
            if rule_value == item[i] {
                count += 1;
            }
        }
        count
    }
}
