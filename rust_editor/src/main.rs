fn num_subarray_product_less_than_k(nums: &[i32], k: i32) -> i32 {
    if k <= 1 {
        return 0;
    }

    let (mut ans, mut left, mut curr) = (0, 0, 1);

    for right in 0..nums.len() {
        curr *= nums[right];

        while curr >= k {
            curr /= nums[left];
            left += 1;
        }

        ans += right - left + 1;
    }

    ans as i32
}

fn main() {
    let nums = vec![10, 5, 2, 6];
    let k = 100;
    let result = num_subarray_product_less_than_k(&nums, k);

    println!("{:?}", result);
}