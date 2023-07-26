
fn main() {
    let yep = Some(42);
    let nope = None;
    // chain()은 이미 `into_iter()`를 호출하므로, 굳이 이렇게 할 필요는 없다.
    let nums: Vec<i32> = (0..4).chain(yep).chain(4..8).collect();
    assert_eq!(nums, [0, 1, 2, 3, 42, 4, 5, 6, 7]);
    let nums: Vec<i32> = (0..4).chain(nope).chain(4..8).collect();
    assert_eq!(nums, [0, 1, 2, 3, 4, 5, 6, 7]);

    let x; // declare "x"
    x = 42; // assign 42 to "x"

    let nick = "pulp-pixel";
    nick.len(); // 10

    println!("{}", nick.len());
}
