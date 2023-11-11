use std::collections::{HashMap, HashSet};

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut dic: HashMap<i32, i32> = HashMap::new();

    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;
        if let Some(&complement_index) = dic.get(&complement) {
            return vec![i as i32, complement_index];
        }

        dic.insert(num, i as i32);
    }

    vec![-1, -1]
}

fn repeated_character_bruce_force(s: &str) -> char {
    for (i, c) in s.chars().enumerate() {
        for j in s[..i].chars() {
            if j == c {
                return c;
            }
        }
    }

    ' ' // 두 번 나타나는 문자가 없는 경우
}

fn repeated_character(s: &str) -> char {
    let mut seen: HashSet<char> = HashSet::new();

    for c in s.chars() {
        if seen.contains(&c) {
            return c;
        }

        seen.insert(c);
    }

    ' '
}

fn find_numbers(nums: Vec<i32>) -> Vec<i32> {
    let nums_set: HashSet<i32> = nums.iter().cloned().collect();
    let mut ans: Vec<i32> = Vec::new();

    for &num in &nums {
        if !nums_set.contains(&(num + 1)) && !nums_set.contains(&(num - 1)) {
            ans.push(num);
        }
    }

    ans
}

fn main() {
    let s = "abccba";
    let result = repeated_character_bruce_force(s);
    let result2 = repeated_character(s);
    println!("{:?}", result);
    println!("{:?}", result2);


    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let result3 = two_sum(nums, target);

    println!("{:?}", result3);

    let nums = vec![1, 3, 5, 7, 8];
    let result4 = find_numbers(nums);
    println!("{:?}", result4);
}
