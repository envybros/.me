---
title: "[Rust] 함수"
categories: [Rust 연구소]
tags: [Rust, Function]
date: 2023-03-06 01:30
img_path: /assets/img/rust/
---

---

![Title](rust_title.png)

---

## **개요**

함수는 Rust에서 많이 사용된다. 가장 기본적인 함수로는 프로그램을 시작하는 `main` 함수가 있다. 함수는 `fn` 키워드를 통해 만들 수 있다.

Rust에서 함수나 변수의 이름을 짓는 방식은 보통 *스네이크 케이스*를 따른다. 여기서 스네이크 케이스란, 모든 글자를 소문자로 쓰고, 단어 사이에 밑줄을 넣는 방식을 뜻한다. 아래 예시를 살펴보자:

파일명: *src/main.rs*

```rs
fn main() {
    println!("Hello, world!");

    another_function();
}

fn another_function() {
    println!("Another function.");
}
```

Rust에서 함수는 `fn` 키워드 뒤에 함수 이름과 괄호를 사용해 정의할 수 있다. 함수의 내용은 중괄호 안에 작성하며, 이 중괄호는 컴파일러에게 해당 함수의 시작과 끝을 알려준다.

정의한 함수는 그 이름 다음에 괄호를 붙여서 호출할 수 있다. 예를 들어, `another_function`은 프로그램 내에 이미 정의되어 있으므로, `main` 함수에서 이를 호출할 수 있다. 소스 코드에서 `main` 함수 **뒤**에 `another_function`을 정의했지만, 필요하다면 `main` 함수 **앞**에 정의할 수도 있다. Rust에서는 함수가 어디에 위치하는지는 중요하지 않다. 중요한 것은 그 함수가 호출될 때 그 위치가 확인될 수 있어야 한다는 것이다.

다양한 함수의 활용을 알아보기 위해 *functions*라는 이름의 새로운 프로젝트를 만들어보자. 위 `another_function` 예제를 *src/main.rs* 파일에 작성한 후 실행하면, 아래와 같은 결과가 출력될 것이다.

```bash
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
    Finished dev [unoptimized + debuginfo] target(s) in 0.28s
     Running `target/debug/functions`
Hello, world!
Another function.
```

`main` 함수 안의 코드는 위에서부터 순서대로 실행된다. 처음에는 "Hello, world!"라는 메시지가 화면에 출력된다. 이후 `another_function` 함수가 호출되면서 그 안의 메시지가 출력된다.

---

## **매개 변수**

함수를 정의할 때 특정 변수들을 함수의 시그니처에 포함시킬 수 있다. 이러한 변수들을 **매개변수**라고 한다. 함수에 매개변수가 있으면, 해당 매개변수에 구체적인 값을 전달할 수 있다. 이렇게 전달되는 구체적인 값을 **인수**라고 한다.

`another_function`이라는 함수의 새로운 버전에 매개변수를 포함시켰다:

파일명: *src/main.rs*

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

함수 `another_function`은 `x`라는 매개변수를 하나 갖고 있다. 이 매개변수 `x`의 자료형은 `i32`로 정의되어 있다. `another_function`에 `5`라는 값이 전달될 때, `println!` 매크로는 포맷 문자열 중 `x`를 대표하는 중괄호 위치에 `5`를 표시한다.

함수를 정의할 때는 각 매개변수의 자료형을 반드시 명시해야 한다. 이는 Rust의 핵심 원칙 중 하나이다. 이 원칙 덕분에 컴파일러는 코드의 다른 부분에서 타입을 추론할 때, 추가적인 정보 없이도 쉽게 파악할 수 있다. 또한, 에러 발생 시 더 정확한 메시지를 제공할 수 있다.

함수 내에서 여러 매개변수를 사용하려면 쉼표로 각 매개변수를 구분하면 된다:

파일명: *src/main.rs*

```rs
fn main() {
    print_labeled_measurement(5, 'h');
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("측정 값은: {value}{unit_label}");
}
```

이 예제에서는 `print_labeled_measurement`라는 함수를 정의한다. 이 함수는 `value`와 `unit_label` 두 개의 매개변수를 받는다. `value`의 자료형은 `i32`이며, `unit_label`의 자료형은 `char`이다. 함수는 이 두 매개변수를 포함하여 출력문을 실행한다.

위의 예제 코드를 실제로 실행해 보려면, *functions* 프로젝트 내의 *src/main.rs* 파일의 내용을 해당 예제로 변경한 후, `cargo run` 명령어로 실행하면 된다.

```bash
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/functions`
측정 값은: 5h
```

`value` 매개변수에 `5`를, `unit_label` 매개변수에 `h`를 인수로 전달하였다. 따라서 프로그램의 출력 결과에 해당 값들이 나타나게 된 것이다.

---

## **구문과 표현식**

Rust에서 함수의 본문은 주로 구문으로 구성되며, 선택적으로 표현식으로 종료될 수 있다. 지금까지 다룬 함수들은 종료 표현식이 포함되어 있지 않았다. 그러나, 구문의 일부로서 표현식을 볼 수 있었다. Rust는 표현식 중심의 언어이므로, 이런 차이를 이해하는 것은 매우 중요하다. 다른 언어들과는 이러한 차이가 다를 수 있으므로, 구문과 표현식이 무엇인지, 그리고 이 두 가지가 함수 본문에 어떻게 영향을 미치는지 살펴보도록 하자.

- **구문**은 어떠한 동작을 수행하는 명령어로, 값을 반환하지 않는다.
- **표현식**은 어떠한 값을 평가하여 결과를 반환한다.

실제로 우리는 이미 구문과 표현식을 사용해왔다. `let` 키워드를 사용하여 변수를 정의하고 그 변수에 값을 할당하는 것은 **구문**에 해당한다. 예를 들어, `let y = 6;`는 구문이다.

파일명: *src/main.rs*

```rs
// 구문 하나를 포함하는 main 함수 선언

fn main() {
    let y = 6;
}
```

이 코드에서 `fn main() {...}`는 함수를 정의하는 구문이다. 그리고 위의 전체 예제는 그 자체로 하나의 큰 구문이다.

구문은 값을 반환하지 않는 특성이 있기 때문에, `let` 구문을 다른 변수에 할당하려고 시도하면 에러가 발생한다.

파일명: *src/main.rs*

> 아래 코드는 컴파일되지 않는다.
{: .prompt-danger }

```rs
fn main() {
    let x = (let y = 6);
}
```

이 프로그램을 실행하면, 다음과 같은 에러 메시지를 볼 수 있다:

```bash
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
error: 표현식을 기대했으나 `let` 구문을 찾았습니다
 --> *src/main.rs*:2:14
  |
2 |     let x = (let y = 6);
  |              ^^^

error: 표현식 위치에서 기대되는 것은 구문(`let`)이 아닙니다
 --> *src/main.rs*:2:14
  |
2 |     let x = (let y = 6);
  |              ^^^^^^^^^
  |
  = 참고: `let`을 사용한 변수 선언은 구문입니다

error[E0658]: 이 위치에서의 `let` 표현식은 안정적이지 않습니다
 --> *src/main.rs*:2:14
  |
2 |     let x = (let y = 6);
  |              ^^^^^^^^^
  |
  = 참고: 더 많은 정보는 이슈 #53667 <https://github.com/rust-lang/rust/issues/53667>를 참조하세요

warning: 할당된 값 주변에 불필요한 괄호가 있습니다
 --> *src/main.rs*:2:13
  |
2 |     let x = (let y = 6);
  |             ^         ^
  |
  = 참고: `#[warn(unused_parens)]`가 기본적으로 활성화되어 있습니다
help: 이 괄호를 제거하세요
  |
2 -     let x = (let y = 6);
2 +     let x = let y = 6;
  |

이 에러에 대한 더 자세한 정보는 `rustc --explain E0658`를 시도해보세요.
경고: `functions` (bin "functions")는 1개의 경고를 생성했습니다
error: 이전 3개의 에러로 인해 `functions`를 컴파일할 수 없습니다; 1개의 경고가 발생했습니다
```

`let y = 6` 구문은 값을 반환하지 않으므로, `x`에 바인딩될 값이 존재하지 않는다. 이는 C나 Ruby 같은 다른 언어들과 차이가 있다. 그 언어들에서는 할당 연산자가 할당된 값을 반환하므로, `x = y = 6`처럼 작성하여 `x`와 `y` 둘 다 `6`의 값을 갖게 할 수 있다. 하지만 Rust에서는 이와 같은 동작을 하지 않는다.

대부분의 Rust 코드는 값을 반환하는 표현식으로 구성된다. 예컨대, `5 + 6`은 `11`이라는 값을 반환하는 표현식이다. 표현식은 구문 내에서도 사용될 수 있다. 위에서 본 `let y = 6;`의 경우, `6`은 `6`이라는 값을 반환하는 표현식이다. 함수나 매크로 호출 또한 표현식에 해당한다. 또한, 중괄호 `{}`를 사용하여 생성된 스코프 블록 역시 표현식으로 간주된다. 예를 들면:

파일명: *src/main.rs*

```rs
fn main() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("y의 값은: {y}");
}
```

위의 표현식:

```rs
{
    let x = 3;
    x + 1
}
```

이 경우, 위의 스코프는 `4`로 평가되며, 해당 값은 `let` 구문의 일부로 `y`에 바인딩된다. 주의할 점은 `x + 1`의 줄 끝에 세미콜론이 없다는 것이다. 이전에 본 대부분의 코드 줄과는 달리, 표현식은 마지막에 세미콜론을 포함하지 않는다. 표현식 끝에 세미콜론을 추가하게 되면 그것은 구문이 되어, 값을 반환하지 않게 된다. 함수의 반환값을 결정하거나 표현식을 다룰 때 이 부분은 특별히 주의해야 한다.

---

## **반환값을 가진 함수**

함수는 값을 반환하여 그 값을 호출한 코드에 전달한다. 반환값의 이름을 별도로 지정할 필요는 없지만, 반환 타입은 화살표(`->`) 다음에 반드시 명시해야 한다. Rust에서는 함수의 마지막 표현식의 값이 그 함수의 반환값으로 처리된다. `return` 키워드를 사용하여 명시적으로 값을 반환하는 것도 가능하지만, 대부분의 함수는 마지막 표현식의 값을 암시적으로 반환한다. 아래는 반환값을 가진 함수의 예시이다.

파일명: *src/main.rs*

```rs
fn five() -> i32 {
    5
}

fn main() {
    let x = five();

    println!("x의 값은: {x}");
}
```

`five` 함수는 숫자 `5`만을 포함하고 있으며, 함수 호출이나 매크로, `let` 구문은 포함되어 있지 않다. 이는 Rust의 특별한 함수 형태이다. 또한, 반환 타입이 `-> i32`로 지정되어 있음을 확인할 수 있다. 위 코드를 실행하면, 다음과 같은 출력 결과를 볼 수 있다:

```bash
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
    Finished dev [unoptimized + debuginfo] target(s) in 0.30s
     Running 'target/debug/functions'
x의 값은: 5
```

`five` 함수 내부의 `5`는 함수의 반환값으로, 이는 반환 타입이 `i32`가 되는 이유이다. 더 깊이 들어가 보면, 첫 번째로, `let x = five();` 구문은 함수의 반환값을 변수에 초기화하는 것을 보여준다. `five` 함수가 `5`를 반환하므로, 해당 구문은 아래와 같이 이해할 수 있다:

```rs
let x = 5;
```

그리고 `five` 함수는 매개변수가 없다. 반환 타입은 정의되어 있지만, 함수 본문 내에는 오직 반환하고자 하는 값의 표현식인 `5`만 존재한다.

다른 예로, 다음의 코드를 살펴보자.

파일명: *src/main.rs*

```rs
fn main() {
    let x = plus_one(5);

    println!("x의 값은: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
```

이 코드를 실행하면 `"x의 값은: 6"`이 출력된다. 하지만, `x + 1` 라인의 끝에 세미콜론을 추가하면, 그것은 표현식에서 구문으로 바뀌게 되고, 이로 인해 에러가 발생한다:

파일명: *src/main.rs*

> 아래 코드는 컴파일되지 않는다.
{: .prompt-danger }

```rs
fn main() {
    let x = plus_one(5);

    println!("x의 값은: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1;
}
```

이 코드를 컴파일하면 다음과 같은 에러가 발생한다:

```bash
$ cargo run
   Compiling functions v0.1.0 (file:///projects/functions)
error[E0308]: mismatched types
 --> src/main.rs:7:24
  |
7 | fn plus_one(x: i32) -> i32 {
  |    --------            ^^^ expected `i32`, found `()`
  |    |
  |    implicitly returns `()` as its body has no tail or `return` expression
8 |     x + 1;
  |          - help: remove this semicolon to return this value

For more information about this error, try `rustc --explain E0308`.
error: could not compile `functions` due to previous error
```

`mismatched types` 에러는 `plus_one` 함수가 `i32` 타입 값을 반환해야 한다는 것을 알려준다. 그러나 현재 코드는 아무 값도 반환하지 않고, `()`(유닛 타입)을 반환한다. 이로 인해 함수의 반환 타입과 충돌이 발생하며 에러가 발생한다. 에러 메시지가 제시한 내용대로 세미콜론을 제거하면 이 문제를 해결할 수 있다.

---

> 출처: [rust-lang book](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)
{: .prompt-info }
