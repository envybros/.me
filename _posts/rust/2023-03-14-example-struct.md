---
title: "[Rust] 구조체를 활용한 예제 프로그램"
categories: [Rust 연구소]
tags: [Rust, Struct]
date: 2023-03-14 03:30
img_path: /assets/img/rust/
---

---

![Title](rust_title.png)

---

## **개요**

직사각형의 면적을 계산하는 프로그램을 작성함으로써 구조체 사용의 필요성을 이해해보자. 초기에는 단일 변수를 사용하다가 이후 프로그램을 리팩토링하여 구조체를 사용하도록 변경할 것이다.

'rectangle'이라는 이름의 새 바이너리 프로젝트를 Cargo를 통해 생성하자. 이 프로젝트는 픽셀 단위로 지정된 직사각형의 너비와 높이를 바탕으로 면적을 계산한다. 아래는 프로젝트의 src/main.rs에 작성된 짧은 프로그램의 예시이다.

파일명: src/main.rs

```rs
// 별도의 너비와 높이 변수로 지정된 직사각형의 면적 계산

fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        "직사각형의 면적은 {} 제곱 픽셀입니다.",
        area(width1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}
```

이제 cargo run 명령어를 사용하여 프로그램을 실행해보자:

```bash
$ cargo run
   Compiling rectangles v0.1.0 (file:///projects/rectangles)
    Finished dev [unoptimized + debuginfo] target(s) in 0.42s
     Running `target/debug/rectangles`
직사각형의 면적은 1500 제곱 픽셀입니다.
```

이 코드는 각 차원에 대한 area 함수 호출을 통해 직사각형의 면적을 계산하지만, 코드를 더 명확하고 읽기 쉽게 만들 수 있다.

문제는 area 함수의 시그니처에 있다:

```rs
fn area(width: u32, height: u32) -> u32 {
```

area 함수는 하나의 직사각형의 면적을 계산하기 위한 것이지만, 우리가 작성한 함수는 두 개의 매개변수를 가지고 있으며, 프로그램의 어디에서도 이 매개변수들이 관련되어 있다는 것이 명확하지 않다. 너비와 높이를 함께 그룹화하는 것이 더 읽기 쉽고 관리하기 쉬울 것이다. 이미 3장의 "튜플 타입" 섹션에서 언급했듯이, 우리는 그렇게 할 수 있는 한 가지 방법을 이미 논의했다: 튜플을 사용하여.

---

> 출처: [rust-lang book](https://doc.rust-lang.org/book/ch05-02-example-structs.html)
{: .prompt-info }
