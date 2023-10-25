use std::io;
use rand::Rng;

fn main() {
    println!("숫자를 맞춰보세요!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("비밀 숫자는: {secret_number}");

    println!("당신이 추측한 숫자를 입력하세요.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("해당 줄을 읽지 못했습니다");

    println!("당신이 추측한 숫자: {guess}");
}
