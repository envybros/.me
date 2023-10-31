---
title: "[Rust] 함수"
categories: [Rust 연구소]
tags: [Rust, Function]
date: 2023-03-05 01:30
math: true
img_path: /assets/img/rust/
---

---

![Title](rust_title.png)

---

## **개요**

Rust에서는 함수를 많이 쓴다. 가장 기본적인 함수로는 프로그램을 시작하는 main 함수가 있다. 함수를 만들 때는 fn 이라는 키워드를 사용한다.

Rust에서 함수나 변수의 이름을 짓는 방식은 보통 스네이크 케이스를 따른다. 이 방식에서는 모든 글자를 소문자로 쓰고, 단어 사이에 밑줄을 넣는다. 아래는 그 예시를 보여주는 프로그램이다.

파일명: src/main.rs

```rs
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}
```

Rust에서 함수는 fn 키워드 뒤에 함수의 이름과 괄호를 사용해 정의한다. 함수의 내용은 중괄호 안에 작성하며, 이 중괄호는 컴파일러에게 해당 함수의 시작과 끝을 알려준다.

정의한 함수는 그 이름 다음에 괄호를 붙여서 호출한다. 예를 들면, another_function은 프로그램 내에 이미 정의되어 있으므로, main 함수에서 이를 호출할 수 있다. 소스 코드에서 main 함수 뒤에 another_function을 정의했지만, 필요하다면 main 함수 앞에 정의할 수도 있다. Rust에서는 함수가 어디에 위치하는지는 중요하지 않다. 중요한 것은 그 함수가 호출될 때 그 위치가 확인될 수 있어야 한다는 것이다.

더 다양한 함수의 활용을 알아보기 위해 functions라는 이름의 새로운 프로젝트를 시작해보자. 위 another_function 예제를 src/main.rs 파일에 작성 후 실행하면, 예상한대로의 결과가 출력될 것이다.

```bash
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
    Finished dev [unoptimized + debuginfo] target(s) in 0.28s
     Running `target/debug/functions`
Hello, world!
Another function.
```

main 함수 안의 코드는 작성된 순서대로 실행된다. 처음에는 "Hello, world!"라는 메시지가 화면에 출력된다. 이후 another_function 함수가 호출되면서 그 안의 메시지가 출력된다.

---

## **매개 변수**

함수를 정의할 때 특정 변수들을 함수의 시그니처에 포함시킬 수 있다. 이러한 변수들을 매개변수라고 한다. 함수에 매개변수가 있으면, 해당 매개변수에 구체적인 값을 전달할 수 있다. 이렇게 전달되는 구체적인 값을 인수라고 한다. 일상적인 대화에서는 함수의 정의에 있는 변수나 함수 호출 시 전달하는 값 모두를 가리켜 매개변수나 인수로 부르는 경우가 많다.

another_function이라는 함수의 새로운 버전에 매개변수를 포함시켰다:

파일명: src/main.rs

```rs
fn main() {
    another_function(5);
}

fn another_function(x: i32) {
    println!("x의 값은: {x}");
}
```

이 프로그램을 실행하면 다음과 같은 결과가 출력된다:

```bash
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
    Finished dev [unoptimized + debuginfo] target(s) in 1.21s
     Running `target/debug/functions`
x의 값은: 5
```

함수 another_function은 x라는 매개변수를 하나 갖고 있다. 이 매개변수 x의 자료형은 i32로 정의되어 있다. another_function에 5라는 값이 전달될 때, println! 매크로는 포맷 문자열 중 x를 대표하는 중괄호 위치에 5를 표시한다.

함수를 정의할 때는 각 매개변수의 자료형을 반드시 명시해야 한다. 이는 Rust의 핵심 원칙 중 하나이다. 이 원칙 덕분에 컴파일러는 코드의 다른 부분에서 타입을 추론할 때, 추가적인 정보 없이도 쉽게 파악할 수 있다. 또한, 에러 발생 시 더 정확한 메시지를 제공할 수 있다.

함수 내에서 여러 매개변수를 사용하려면 쉼표로 각 매개변수를 구분하면 된다:

파일명: src/main.rs

```rs
fn main() {
    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("측정 값은: {value}{unit_label}");
}
```

이 예제에서는 print_labeled_measurement라는 함수를 정의한다. 이 함수는 value와 unit_label 두 개의 매개변수를 받는다. value의 자료형은 i32이며, unit_label의 자료형은 char이다. 함수는 이 두 매개변수를 포함하여 출력문을 실행한다.

위의 예제 코드를 실제로 실행해 보려면, functions 프로젝트 내의 src/main.rs 파일의 내용을 해당 예제로 변경한 후, cargo run 명령어로 실행하면 된다.

```bash
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/functions`
측정 값은: 5h
```

value 매개변수에 5를, unit_label 매개변수에 'h'를 인수로 전달하였다. 따라서 프로그램의 출력 결과에 해당 값들이 나타나게 된다.

---

## **구문과 표현식**

Rust에서 함수의 본문은 여러 구문으로 구성되며, 선택적으로 표현식으로 끝날 수 있다. 지금까지 살펴본 함수들은 종료 표현식을 포함하고 있지 않았지만, 구문의 일부로 표현식을 볼 수 있었다. Rust는 표현식 기반의 언어이기 때문에, 이러한 구분을 이해하는 것이 중요하다. 다른 언어들은 이와 같은 구분을 가지고 있지 않다. 그렇기에 구문과 표현식이 무엇인지 그리고 그들의 차이가 함수의 본문에 어떠한 영향을 주는지에 대해 알아보도록 하자.

- 구문: 특정 동작을 수행하는 지시어이며, 값을 반환하지 않는다.
- 표현식: 값을 평가하여 그 결과를 반환한다. 몇 가지 예를 통해 확인해보자.

우리는 이미 구문과 표현식을 사용해 왔다. let 키워드를 사용해 변수를 생성하고 그에 값을 할당하는 것은 구문이다. 아래 코드에서 let y = 6;은 구문의 예시이다.

파일명: src/main.rs

```rust
// 하나의 문을 포함하는 main 함수 선언

fn main() {
    let y = 6;
}
```

함수의 정의 또한 구문이다. 위의 예시 전체는 자체로서의 구문이다.

구문은 값을 반환하지 않기 때문에, 다음 코드와 같이 let 구문을 다른 변수에 할당하려고 하면 에러가 발생한다:

파일명: src/main.rs

```rs
fn main() {
    let x = (let y = 6);
}
```

이 프로그램을 실행하면, 다음과 같은 에러 메시지가 나타난다:

```bash
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
error: expected expression, found `let` statement
 --> src/main.rs:2:14
  |
2 |     let x = (let y = 6);
  |              ^^^

error: expected expression, found statement (`let`)
 --> src/main.rs:2:14
  |
2 |     let x = (let y = 6);
  |              ^^^^^^^^^
  |
  = note: variable declaration using `let` is a statement

error[E0658]: `let` expressions in this position are unstable
 --> src/main.rs:2:14
  |
2 |     let x = (let y = 6);
  |              ^^^^^^^^^
  |
  = note: see issue #53667 <https://github.com/rust-lang/rust/issues/53667> for more information

warning: unnecessary parentheses around assigned value
 --> src/main.rs:2:13
  |
2 |     let x = (let y = 6);
  |             ^         ^
  |
  = note: `#[warn(unused_parens)]` on by default
help: remove these parentheses
  |
2 -     let x = (let y = 6);
2 +     let x = let y = 6;
  |

For more information about this error, try `rustc --explain E0658`.
warning: `functions` (bin "functions") generated 1 warning
error: could not compile `functions` due to 3 previous errors; 1 warning emitted
```

let y = 6; 구문은 값을 반환하지 않기 때문에, x에 바인딩될 값이 없다. 이러한 특성은 C나 Ruby와 같은 다른 언어들과는 다르다. 그 언어들에서는 할당 연산이 할당된 값을 반환한다. 따라서 그러한 언어에서는 x = y = 6과 같이 작성하여 x와 y 모두에 6의 값을 가지게 할 수 있다. 그러나 Rust에서는 그렇지 않다.

대부분의 Rust 코드를 구성하는 나머지 부분은 표현식으로 평가되어 값을 반환한다. 예를 들어, 5 + 6과 같은 수학 연산은 11이라는 값을 반환하는 표현식이다. 표현식은 구문의 일부가 될 수도 있다. 위 코드에서 let y = 6; 구문의 6은 6이라는 값을 반환하는 표현식이다. 함수를 호출하는 것은 표현식이다. 매크로를 호출하는 것 또한 표현식이다. 중괄호로 생성된 새로운 스코프 블록 역시 표현식이다, 예를 들면:

파일명: src/main.rs

```rs
fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("y의 값은: {y}");
}
```

이러한 표현식:

```rs
{
    let x = 3;
    x + 1
}
```

이 경우에 위 스코프는 4로 평가된다. 그 값은 let 구문의 일부로 y에 바인딩된다. 주목해야 할 것은 x + 1의 줄 끝에 세미콜론이 없다는 점이다. 이는 지금까지 본 대부분의 줄들과는 다르다. 표현식은 마지막에 세미콜론을 포함하지 않는다. 표현식의 끝에 세미콜론을 추가하면, 그것은 구문이 되고, 그 결과로 값을 반환하지 않게 된다. 이는 함수의 반환 값과 표현식을 탐구할 때 특히 유의해야 하는 부분이다.

---

## 

---

> 출처: [rust-lang book](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
{: .prompt-info }
