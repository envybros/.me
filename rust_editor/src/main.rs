fn combine(arr1: Vec<i32>, arr2: Vec<i32>) -> Vec<i32> {
    let mut ans = Vec::new();
    let (mut i, mut j) = (0, 0);

    while i < arr1.len() && j < arr2.len() {
        if arr1[i] < arr2[j] {
            ans.push(arr1[i]);
            i += 1;
        } else {
            ans.push(arr2[j]);
            j += 1;
        }
    }

    while i < arr1.len() {
        ans.push(arr1[i]);
        i += 1;
    }

    while j < arr2.len() {
        ans.push(arr2[j]);
        j += 1;
    }

    ans
}

fn main() {
    let vec = vec![-4,-1,0,3,10];
    let vec2 = vec![-5,-3,1,2,4];

    println!("{:?}", combine(vec, vec2));
}