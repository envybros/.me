fn find_length(nums: Vec<i32>, k: i32) -> i32 {
    let (mut left, mut curr, mut ans) = (0, 0, 0);

    for right in 0..nums.len() {
       curr += nums[right];
        while curr > k {
            curr -= nums[left];
            left += 1;
        }

        ans = std::cmp::max(ans, (right as i32) - (left as i32) + 1);
    }

    ans
}

fn main() {
    let nums = vec![1, 2, 3, 4, 5];
    let k = 6;
    let result = find_length(nums, k);

    println!("{:?}", result);
}