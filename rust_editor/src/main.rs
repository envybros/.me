fn ways_to_split_array(nums: Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut left_section = 0;
    let total: i32 = nums.iter().sum();

    for i in 0..nums.len() - 1 {
        left_section += nums[i];
        let right_section = total - left_section;
        if left_section >= right_section {
            ans += 1;
        }
    }

    ans
}

fn main() {
    let nums = vec![1, 2, 3, 4];
    let result = ways_to_split_array(nums);

    println!("{:?}", result);
}
