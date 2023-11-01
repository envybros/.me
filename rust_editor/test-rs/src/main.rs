// 조건이 true인 동안 코드를 실행하기 위해 while 루프 사용

fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}