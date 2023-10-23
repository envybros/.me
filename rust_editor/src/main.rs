fn find_max_average(nums: Vec<i32>, k: i32) -> f64 {
    let k_len: usize = k as usize;

    let mut curr: i32 = nums.iter().take(k_len).sum();

    let mut ans: f64 = curr as f64 / k as f64;
    for i in k_len..nums.len() {
        curr += nums[i] - nums[i-k_len];
        ans = ans.max(curr as f64 / k as f64);
    }

    ans
}

fn main() {
    let nums = vec![1,12,-5,-6,50,3];
    let k = 4;
    let result = find_max_average(nums, k);

    println!("{:?}", result);
}