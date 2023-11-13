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

fn find_longest_substring(s: &str, k: usize) -> usize {
    let mut counts: HashMap<char, i32> = HashMap::new();
    let (mut left, mut ans) = (0, 0);

    for (right, c) in s.chars().enumerate() {
        *counts.entry(c).or_insert(0) += 1;

        while counts.len() > k {
            let left_char = s.chars().nth(left).unwrap();
            *counts.get_mut(&left_char).unwrap() -= 1;

            if counts[&left_char] == 0 {
                counts.remove(&left_char);
            }

            left += 1;
        }

        ans = ans.max(right - left + 1);
    }

    ans
}

fn intersection(nums: Vec<Vec<i32>>) -> Vec<i32> {
    let mut counts = HashMap::new();

    for arr in nums.iter() {
        for &x in arr {
            *counts.entry(x).or_insert(0) += 1;
        }
    }

    let n = nums.len() as i32;
    let mut ans: Vec<i32> = counts.into_iter()
        .filter(|&(_key, val)| val == n)
        .map(|(key, _val)| key)
        .collect();

    ans.sort_unstable();
    ans
}

fn are_occurrences_equal(s: &str) -> bool {
    let mut counts: HashMap<char, i32> = HashMap::new();

    for c in s.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }

    let frequencies: HashSet<&i32> = counts.values().collect();

    frequencies.len() == 1
}

fn subarray_sum(nums: &[i32], k: i32) -> i32 {
    let mut counts = HashMap::new();

    counts.insert(0, 1);
    let (mut ans, mut curr) = (0, 0);

    for &num in nums {
        curr += num;
        ans += counts.get(&(curr - k)).unwrap_or(&0);
        *counts.entry(curr).or_insert(0) += 1;
    }

    ans
}

fn number_of_subarrays(nums: &[i32], k: i32) -> i32 {
    let mut counts = HashMap::new();
    counts.insert(0, 1);
    let (mut ans, mut curr) = (0, 0);

    for &num in nums {
        curr += num % 2;
        ans += *counts.get(&(curr - k)).unwrap_or(&0);
        *counts.entry(curr).or_insert(0) += 1;
    }

    ans
}

fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut group: HashMap<String, Vec<String>> = HashMap::new();

    for s in strs.iter() {
        let mut t = s.clone();
        let mut chars: Vec<char> = t.chars().collect();
        chars.sort_unstable();
        t = chars.into_iter().collect();

        group.entry(t).or_insert(Vec::new()).push(s.clone());
    }

    group.into_values().collect()
}

fn minimum_card_pickup(cards: Vec<i32>) -> i32 {
    let mut dic: HashMap<i32, Vec<usize>> = HashMap::new();

    for (i, &card) in cards.iter().enumerate() {
        dic.entry(card).or_default().push(i);
    }

    let mut ans = i32::MAX;
    for arr in dic.values() {
        for window in arr.windows(2) {
            if let [a, b] = window {
                ans = ans.min((b - a) as i32 + 1);
            }
        }
    }

    if ans == i32::MAX { -1 } else { ans }
}

fn minimum_card_pickup_optimized(cards: Vec<i32>) -> i32 {
    let mut dic: HashMap<i32, usize> = HashMap::new();
    let mut ans = i32::MAX;

    for (i, &card) in cards.iter().enumerate() {
        if let Some(&last_index) = dic.get(&card) {
            ans = ans.min(i as i32 - last_index as i32 + 1);
        }
        dic.insert(card, i);
    }

    if ans == i32::MAX { -1 } else { ans }
}

fn main() {
    let cards = vec![1, 2, 6, 2, 1];
    let result = minimum_card_pickup_optimized(cards);
    println!("{:?}", result);

    let cards = vec![1, 2, 6, 2, 1];
    let result = minimum_card_pickup(cards);
    println!("{:?}", result);

    let strs = vec![
        "eat".to_string(),
        "tea".to_string(),
        "tan".to_string(),
        "ate".to_string(),
        "nat".to_string(),
        "bat".to_string(),
    ];

    let result = group_anagrams(strs);
    println!("{:?}", result);

    let nums = vec![1, 1, 2, 1, 1];
    let k = 3;
    let result = number_of_subarrays(&nums, k);
    println!("{:?}", result); // 출력 예시: 2

    // 추가 테스트 케이스
    let nums = vec![2, 4, 6, 8, 10];
    let k = 1;
    let result = number_of_subarrays(&nums, k);
    println!("{:?}", result); // 출력 예시: 0

    let nums = vec![1, 2, 1, 2, 1];
    let k = 3;
    let result = subarray_sum(&nums, k);
    println!("{:?}", result); // 출력 예시: 4

    // 추가 테스트 케이스
    let nums = vec![1, -1, 1, -1];
    let k = 0;
    let result = subarray_sum(&nums, k);
    println!("{:?}", result); // 출력 예시: 4

    let s = "abacbc";
    let result = are_occurrences_equal(s);
    println!("{:?}", result); // 출력 예시: true

    let s = "aaabb";
    let result = are_occurrences_equal(s);
    println!("{:?}", result); // 출력 예시: false

    let nums = vec![vec![3, 1, 2, 4, 5], vec![1, 2, 3, 4], vec![3, 4, 5, 6]];
    let result = intersection(nums);

    println!("{:?}", result);

    let s = "eceba";
    let k = 2;
    let result = find_longest_substring(s, k);

    println!("{:?}", result); // 출력 예시: 3

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
