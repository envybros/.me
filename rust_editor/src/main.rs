use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("숫자를 맞춰보세요!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("당신이 추측한 숫자를 입력하세요.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("해당 줄을 읽지 못했습니다.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("당신이 추측한 숫자: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("숫자가 너무 작습니다!!"),
            Ordering::Greater => println!("숫자가 너무 큽니다!!"),
            Ordering::Equal => {
                println!("숫자를 맞췄습니다!");
                break;
            }
        }
    }
}