---
title: "[Rust] 제어 흐름"
categories: [Rust 연구소]
tags: [Rust, Control-flow]
date: 2023-03-08 02:30
math: true
img_path: /assets/img/rust/
---

---

![Title](rust_title.png)

---

## **개요**

대부분의 프로그래밍 언어는 조건이 참인 경우 코드를 실행하거나, 조건이 계속 참인 동안 코드를 반복해서 실행하는 기능을 제공한다. Rust에서는 if 표현식과 루프로 실행 흐름을 제어한다.

## **if 표현식**

if 표현식으로 조건에 따른 코드의 실행 여부를 결정할 수 있다. "조건이 참일 때 해당 코드 블록을 실행하며, 조건이 거짓일 때는 실행하지 않는다."로 이해할 수 있다.

projects 디렉토리에 'branches'라는 새 프로젝트를 생성하고, if 표현식을 살펴보자. *src/main.rs* 파일에 다음 내용을 작성해보자.

파일명: *src/main.rs*

```rs
fn main() {
    let number = 3;

    if number < 5 {
        println!("조건은 true이다.");
    } else {
        println!("조건은 false이다.");
    }
}
```

if 표현식은 항상 'if' 키워드로 시작하며, 그 뒤에 조건이 따라온다. 예를 들어, 위의 예제에서는 'number' 변수가 5보다 작은지 확인하는 조건을 사용했다. 조건이 참일 때 실행되는 코드 블록은 조건 다음 중괄호 내에 위치한다. if 표현식의 이러한 코드 블록을 분기(arms)라 부른다. 이는 예전에 다뤘던 match 표현식의 분기와 관련이 있다.

조건이 거짓일 경우 실행될 대안 코드는 'else' 표현식으로 작성할 수 있다. 'else' 표현식을 작성하지 않을 경우, 조건이 거짓이면 if 블록을 건너뛰고, 다음 코드로 이동한다.

위 코드를 실행하면, 아래와 같은 결과가 출력된다:

```bash
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/branches`
조건은 true이다.
```

변수 'number'의 값을 변경하여 조건이 거짓일 때 어떤 반응이 나타나는지 확인해보자.

```rs
    let number = 7;
```

프로그램을 다시 실행하면 다음과 같은 결과를 확인할 수 있다:

```bash
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31s
     Running 'target/debug/branches'
조건은 false이다.
```

조건문의 조건은 반드시 불리언 타입(bool)이어야 한다. 불리언 타입이 아닌 값으로 조건을 설정하면 에러가 발생한다. 아래 코드를 보면 더 잘 이해할 수 있다:

파일명: *src/main.rs*

> 아래 코드는 컴파일되지 않는다.
{: .prompt-danger }

```rs
fn main() {
    let number = 3;

    if number {
        println!("number는 3이다.");
    }
}
```

위 코드에서는 if 조건이 3의 값으로 평가되었고, Rust에서는 에러를 반환한다:

```bash
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
error[E0308]: mismatched types
 --> src/main.rs:4:8
  |
4 |     if number {
  |        ^^^^^^ expected `bool`, found integer

For more information about this error, try `rustc --explain E0308`.
error: could not compile `branches` due to previous error
```

에러 메시지는 Rust가 불리언 값을 기대했으나 정수 값을 받았다는 것을 나타낸다. Ruby나 JavaScript와 같은 다른 언어들과 달리 Rust는 자동으로 불리언이 아닌 타입을 불리언으로 변환하지 않는다. if 문에는 반드시 불리언 값을 조건으로 제공해야 한다. 예를 들어, 숫자가 0이 아닐 때만 if 코드 블록이 실행되도록 하려면, if 표현식을 아래와 같이 변경할 수 있다:

파일명: *src/main.rs*

```rs
fn main() {
    let number = 3;

    if number != 0 {
        println!("number는 0이 아니다.");
    }
}
```

이 코드를 실행하면 "number는 0이 아니다."가 출력된다.

## **else if를 활용한 다중 조건 처리**

else if 표현식을 사용하면 여러 조건을 처리할 수 있다. 아래 예제를 통해 확인해보자:

파일명: src/main.rs

```rs
fn main() {
    let number = 6;

    if number % 4 == 0 {
        println!("number는 4로 나누어 떨어진다");
    } else if number % 3 == 0 {
        println!("number는 3으로 나누어 떨어진다");
    } else if number % 2 == 0 {
        println!("number는 2로 나누어 떨어진다");
    } else {
        println!("number는 4, 3, 2로 나누어 떨어지지 않는다");
    }
}
```

위 프로그램은 네 가지의 경로를 가진다. 이를 실행하면 다음과 같은 결과가 출력된다:

```bash
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31s
     Running 'target/debug/branches'
number는 3으로 나누어 떨어진다
```

프로그램은 각 if 조건을 순서대로 검사하고, 참인 첫 번째 조건의 코드 블록을 실행한다. 비록 6이 2로 나누어 떨어지지만 "number는 2로 나누어 떨어진다"는 메시지는 출력되지 않는다. 이는 Rust가 참인 첫 번째 조건을 만나면 이후 조건들을 확인하지 않기 때문이다.

다수의 else if 조건 사용은 코드의 복잡성을 증가시킬 수 있다. 여러 조건들이 존재할 때는 코드를 리팩터링해야 한다. 추후에 Rust가 제공하는 강력한 분기 처리 도구인 match에 대해 알아볼 것이다.

---

## **let 문에서 if 사용하기**

if는 표현식이므로, let 문의 오른쪽에서 사용할 수 있다. 이를 통해 변수에 결과 값을 할당할 수 있다. 아래 코드에서 예시를 확인할 수 있다.

파일명: *src/main.rs*

```rs
fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("number의 값은: {number}이다.");
}
```

위 코드에서 number 변수는 if 표현식의 결과에 따라 값이 할당된다. 아래의 명령어를 통해 실행한 결과를 확인하면 number의 값은 5로 할당된 것을 알 수 있다.

```bash
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
    Finished dev [unoptimized + debuginfo] target(s) in 0.30s
     Running `target/debug/branches`
number의 값은: 5이다.
```

코드의 블록은 그 안의 마지막 표현식의 값으로 평가된다. 따라서 전체 if 표현식의 값은 실행되는 코드 블록에 따라 결정된다. 이 때, if의 각 분기에서의 결과 값은 동일한 타입을 가지고 있어야 한다. 만약 타입이 일치하지 않으면, 에러가 발생한다.

> 아래 코드는 컴파일되지 않는다.
{: .prompt-danger }

```rs
fn main() {
    let condition = true;
    let number = if condition { 5 } else { "six" };

    println!("number의 값은: {number}이다.");
}
```

위 코드를 컴파일하려고 시도하면, if와 else의 반환 타입이 서로 호환되지 않아 에러가 발생한다.

```bash
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
error[E0308]: `if`와 `else`의 타입이 호환되지 않는다.
 --> src/main.rs:4:44
  |
4 |     let number = if condition { 5 } else { "six" };
  |                                 -          ^^^^^ 정수를 예상했으나 `&str` 타입 발견
  |                                 |
  |                                 이 부분 때문에 발생한 문제

에러에 대한 자세한 정보는 `rustc --explain E0308`를 통해 확인할 수 있다.
error: 이전 에러로 인해 `branches` 컴파일에 실패했다.
```

if 문 내의 표현식은 정수로 평가되고, else 문 내의 표현식은 문자열로 평가된다. 변수는 하나의 명확한 타입을 가져야 하며, 이 타입은 컴파일 시간에 결정되어야 한다. 이를 통해 컴파일러는 number의 타입이 올바른지 코드 내의 모든 위치에서 검증할 수 있다.

---

> 출처: [rust-lang book](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
{: .prompt-info }
