use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    // 1
    pub fn dest_city(paths: Vec<Vec<String>>) -> String {
        let mut path_hash = HashMap::new();
        for path in paths.iter() {
            path_hash.insert(path.get(0).unwrap(), path.get(1).unwrap());
        }
        for path in paths.iter() {
            let des = path.get(1).unwrap();
            if !path_hash.contains_key(des) {
                return des.to_string();
            }
        }
        return String::from("222");
    }

    // 2

    pub fn count_beautiful_pairs(nums: Vec<i32>) -> i32 {
        let mut r = 0;
        for i in 0..nums.len() - 1 {
            for j in (i + 1)..nums.len() {
                if find_gcd(nums[i], nums[j]) == 1 {
                    r = r + 1;
                }
            }
        }
        r
    }
}

pub fn find_gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let remainder = a % b;
        a = b;
        b = remainder;
    }
    a
}
