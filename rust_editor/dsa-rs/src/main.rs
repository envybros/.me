use std::collections::HashMap;

fn maximum_sum_optimized(nums: Vec<i32>) -> i32 {
    let mut dic: HashMap<i32, i32> = HashMap::new();
    let mut ans = -1;

    for &num in &nums {
        let digit_sum = get_digit_sum(num);
        let prev_num = dic.get(&digit_sum).unwrap_or(&0);
        ans = ans.max(num + prev_num);

        // 여기서 `max` 메소드에 직접 값을 전달
        dic.insert(digit_sum, *prev_num.max(&num));
    }

    ans
}

fn get_digit_sum(mut num: i32) -> i32 {
    let mut digit_sum = 0;
    while num > 0 {
        digit_sum += num % 10;
        num /= 10;
    }

    digit_sum
}

fn main() {
    let nums = vec![51, 71, 17, 42];
    let result = maximum_sum_optimized(nums);
    println!("{:?}", result);
}
