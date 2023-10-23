fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut ans = vec![0; n];

    let (mut left, mut right) = (0, n-1);

    for i in (0..n).rev() {
        if nums[left].abs() > nums[right].abs() {
            ans[i] = nums[left] * nums[left];
            left += 1;
        } else {
            ans[i] = nums[right] * nums[right];
            right -= 1;
        }
    }

    ans
}

fn main() {
    let vec = vec![-4,-1,0,3,10];

    println!("{:?}", sorted_squares(vec));
}