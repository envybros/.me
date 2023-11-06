---
title: "[Rust] 참조와 빌림"
categories: [Rust 연구소]
tags: [Rust, Control-flow]
date: 2023-03-11 01:30
img_path: /assets/img/rust/
---

---

![Title](rust_title.png)

---

## **개요**

```rs
// 매개변수의 소유권 반환에 관한 예제

fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("'{}'의 길이는 {}입니다.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();  // len()은 String의 길이를 반환한다

    (s, length)
}
```

위 코드에서 `calculate_length` 함수를 호출한 후에도 `String`을 사용하고 싶다면, 호출된 함수에서 `String`을 반환해야 한다. 이렇게 해야 하는 이유는 `String`이 `calculate_length`로 넘어가면서 소유권이 이동되었기 때문이다. 여기서, `String`의 참조를 넘겨줌으로써 해당 값을 계속 사용하게 할 수 있다. 참조는 메모리 상의 특정 주소를 가리키며, 그 주소에 있는 데이터에 접근할 수 있게 해주는데, 이 데이터는 다른 변수가 소유하고 있다. 포인터와 달리 참조는 사용되는 동안 항상 유효한 타입의 값을 가리킨다는 것이 보장된다.

이제 객체를 넘기는 대신 참조를 매개변수로 사용하는 `calculate_length` 함수의 정의 및 사용 방법을 살펴보자:

파일명: *src/main.rs*

```rs
fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("'{}'의 길이는 {}이다.", s1, len);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}
```

변수를 선언하고 함수의 반환값을 정의할 때 튜플을 사용하던 코드는 이제 보이지 않는다. `calculate_length` 함수에 `&s1`을 전달할 때, 그리고 그 함수의 매개변수 타입으로 `String` 대신 `&String`을 사용할 때 이 앰퍼샌드(`&`) 기호가 중요한 역할을 한다. 이 기호는 **참조**를 나타내며, 값을 소유하지 않고도 그 값을 참조할 수 있음을 의미한다. 다음은 이 개념을 도식화한 그림이다.

> ![refandbor00](refandbor00.png)
>
> [그림 1-1] `&String s`가 `String s1`을 가리키는 다이어그램
{: .prompt-general }

---

> 참고: 참조를 나타내는 `&`와는 반대되는 개념으로 역참조가 있다. 역참조는 `*` 연산자를 사용해서 이루어진다.
{: .prompt-tip }

---

이제 함수 호출 과정에 대해 더 자세히 알아보자:

```rs
let s1 = String::from("hello");

let len = calculate_length(&s1);
```

위 코드에서는 `&s1`을 통해 `s1`의 값을 소유하지 않으면서도 **참조**를 만들 수 있다. 그래서 참조를 더 이상 사용하지 않더라도, 참조가 가리키고 있는 값은 메모리에서 해제되지 않는다.

함수 정의에서도 `&` 기호를 사용해서 매개변수 `s`가 참조임을 명시한다. 주석 내용을 참고하며 아래 코드를 살펴보자:

```rs
// `calculate_length` 함수는 `String`의 참조를 매개변수로 받고 길이를 반환한다.
fn calculate_length(s: &String) -> usize { // s는 String에 대한 참조
    s.len()
} // 여기서 `s`의 범위가 종료되지만,
  // `s`가 가리키는 값은 해제되지 않는다.
  // 이는 `s`에게 소유권이 없기 때문이다.
```

변수 `s`의 범위는 다른 함수 매개변수의 범위와 동일하다. 그러나 참조가 가리키는 값은 `s`의 사용이 종료될 때 해제되지 않는다. 이는 `s`가 해당 값을 소유하고 있지 않기 때문이다. 실제 값의 복사본을 만들어 소유권을 전달하는 대신에 참조를 매개변수로 사용하는 함수는 소유권을 반환할 필요가 없다.

참조를 만드는 이 과정을 **빌림**이라고 한다. 물건을 소유하고 있는 사람으로부터 그것을 빌려 사용한 후 사용이 종료되면 그것을 반납하는 것과 유사하다.

만약 빌린 값을 변경하려고 할 경우, 어떤 결과가 발생할까?

파일명: *src/main.rs*

> 아래 코드는 컴파일되지 않는다.
{: .prompt-danger }

```rs
// 빌린 값에 변경을 가하려고 할 때

fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}
```

이 코드를 컴파일하려고 하면 다음과 같은 에러 메시지가 나타난다:

```bash
$ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0596]: `*some_string`을 가변으로 빌리고자 하였으나, `&` 참조를 통해서는 불가능하다.
 --> src/main.rs:8:5
  |
7 | fn change(some_string: &String) {
  |                        ------- 도움말: 가변 참조로 변경하고자 한다면 `&mut String`을 고려하라.
8 |     some_string.push_str(", world");
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `some_string`이 `&` 참조이기 때문에, 가변으로 빌리는 것이 허용되지 않는다.

에러에 대한 자세한 정보를 원하신다면, rustc --explain E0596을 실행하라.
error: 발생한 에러로 인하여 ownership 프로젝트의 컴파일을 완료할 수 없다.
```

변수가 기본적으로 불변임을 원칙으로 하듯, 참조도 불변이다. 따라서 참조를 통해 값을 변경하는 것은 허용되지 않는다.

---

## **가변 참조**

위 코드를 수정하여 빌린 값을 변경할 수 있도록 하려면, **가변 참조**를 사용해야 한다.

파일명: *src/main.rs*

```rs
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

이 코드에서는 `s`를 변경 가능하게(`mut`) 선언하였다. 또한 `change` 함수를 호출할 때 `&mut s`를 사용하여 가변 참조를 생성하고, 함수의 매개변수 타입도 `some_string: &mut String`으로 변경하여 가변 참조를 받을 수 있도록 하였다. 이제 `change` 함수 내에서 `some_string`의 값을 수정할 수 있다.

가변 참조에는 중요한 제약이 있다: 한 번에 하나의 가변 참조만이 허용된다. 즉, 동일한 데이터에 대해 여러 개의 가변 참조를 동시에 생성하는 것은 허용되지 않는다.

다음 코드에서는 `s`에 대한 두 개의 가변 참조를 생성하려고 시도하고 있다:

파일명: *src/main.rs*

> 아래 코드는 컴파일되지 않는다.
{: .prompt-danger }

```rs
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);
```

이 코드를 실행하면 다음과 같은 에러 메시지가 나타난다:

```bash
$ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0499]: `s`를 한 번에 두 번 이상 가변으로 빌릴 수 없다.
 --> src/main.rs:5:14
  |
4 |     let r1 = &mut s;
  |              ------ 여기서 첫 번째 가변 빌림이 발생한다.
5 |     let r2 = &mut s;
  |              ^^^^^^ 여기서 두 번째 가변 빌림이 발생한다.
6 |
7 |     println!("{}, {}", r1, r2);
  |                        -- 첫 번째 빌림은 여기서 사용된다.

자세한 정보를 원한다면, rustc --explain E0499을 실행하라.
error: 발생한 에러로 인하여 ownership 프로젝트의 컴파일을 완료할 수 없다.
```

이 에러 메시지는 `s` 변수가 동시에 두 개의 가변 참조를 가질 수 없음을 알려준다. 첫 번째 가변 참조는 `r1`에서, 두 번째는 `r2`에서 나타난다.

한 데이터에 대해 동시에 여러 가변 참조를 만들지 못하게 하는 이 규칙은 데이터의 변형을 체계적으로 통제한다. 다른 언어들은 개발자가 원할 때 언제든지 데이터를 변형할 수 있게 하지만, Rust는 이를 제한함으로써 특별한 장점을 제공한다. 이러한 제한은 Rust가 컴파일 단계에서 데이터 경쟁을 사전에 차단할 수 있게 해준다. 데이터 경쟁은 다음 세 조건이 충족될 때 발생하는 문제이다:

- 두 개 이상의 포인터가 동시에 같은 데이터에 접근한다.
- 이 중 최소 한 개의 포인터가 데이터에 쓰기를 시도한다.
- 데이터 접근을 조율하는 동기화 메커니즘이 없다.

데이터 경쟁은 예측 불가능한 동작을 초래하며, 런타임 때 이를 발견하고 수정하기 어렵다. Rust는 이러한 문제를 컴파일 과정에서 데이터 경쟁을 일으킬 수 있는 코드를 배제함으로써 해결한다.

또한, 중괄호를 사용해 새로운 범위를 만들어주면 동시에 일어나지 않는 여러 가변 참조를 허용할 수 있다.

```rs
let mut s = String::from("hello");

{
    let r1 = &mut s;
} // 여기서 r1의 스코프가 끝나므로, 문제 없이 새 참조를 만들 수 있다.

let r2 = &mut s;
```

Rust에서는 가변 참조와 불변 참조를 함께 사용할 때도 비슷한 규칙을 적용한다.

> 아래 코드는 컴파일되지 않는다.
{: .prompt-danger }

```rs
let mut s = String::from("hello");

let r1 = &s; // 이것은 괜찮다
let r2 = &s; // 이것도 문제가 없다
let r3 = &mut s; // 여기서 큰 문제가 발생한다

println!("{}, {}, 그리고 {}", r1, r2, r3);
```

이때 발생하는 에러 메시지는 다음과 같다:

```bash
$ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0502]: `s`에 대해 불변으로 빌린 상태에서 가변으로 빌릴 수 없다.
 --> *src/main.rs*:6:14
  |
4 |     let r1 = &s; 
  |              -- 불변 빌림 발생
5 |     let r2 = &s; 
6 |     let r3 = &mut s; 
  |              ^^^^^^ 가변 빌림 발생
7 |
8 |     println!("{}, {}, 그리고 {}", r1, r2, r3);
  |                                -- 불변 빌림이 여기서 사용됨

이 에러에 대한 자세한 정보를 보려면 `rustc --explain E0502`를 확인한다.
error: 이전의 에러 때문에 `ownership` 컴파일에 실패하였다.
```

우리는 동시에 한 값에 대한 불변 참조와 가변 참조를 가질 수 없다는 것을 알 수 있다.

불변 참조를 사용하는 개발자들은 그 값이 예상치 못하게 변경되지 않기를 원한다. 여러 불변 참조가 있어도 문제가 없는 이유는, 이들이 데이터를 읽기만 하기 때문에 서로 방해하지 않기 때문이다.

참조의 유효 범위란 그 참조가 시작되는 지점부터 마지막으로 사용되는 지점까지를 말한다. 예를 들어, 불변 참조가 끝나고 난 뒤에(`println!`), 가변 참조가 시작되는 아래 코드는 문제없이 컴파일될 수 있다.

```rs
let mut s = String::from("hello");

let r1 = &s; // 이건 괜찮다
let r2 = &s; // 이것도 문제없다
println!("{} 그리고 {}", r1, r2);
// 여기서 r1과 r2는 더 이상 사용되지 않는다

let r3 = &mut s; // 이것도 괜찮다
println!("{}", r3);
```

불변 참조인 `r1`과 `r2`는 가변 참조 `r3`가 생기기 전, 그들이 마지막으로 쓰인 `println!` 이후에 유효 범위가 끝난다. 이 두 범위는 겹치지 않으므로 이 코드는 문제없이 작동한다. 컴파일러는 참조가 더 이상 쓰이지 않는 시점을 알고 있기 때문에 범위가 어디서 끝나는지 알 수 있다.

Rust에서 빌림 관련 에러가 나타날 때마다, 이것이 런타임 전에 컴파일러가 잠재적인 문제를 지적하고 정확히 어떤 문제인지 알려주는 것임을 기억하자. 이를 통해 개발자는 왜 데이터가 자신이 예상한 것과 다른지 궁금해할 필요가 없다.

---

## **허상 참조**

포인터를 사용하는 프로그래밍 언어에서는, 메모리를 사용하다가 더 이상 필요 없어서 해제했는데도 그 메모리를 가리키는 포인터를 그대로 두면 문제가 생길 수 있다. 이런 상황에서 그 메모리가 다른 용도로 다시 쓰이게 되면, **허상 참조**라고 불리는 오류가 발생할 수 있다. 이는 포인터가 실제로는 없어진 메모리를 가리키고 있기 때문이다. 하지만 Rust에서는 이런 일이 발생하지 않는다. Rust의 컴파일러는 프로그램을 만들 때 이런 허상 참조가 생기지 않도록 철저히 확인한다. 즉, 데이터를 참조하고 있다면, 그 데이터가 필요한 동안은 사라지지 않도록 컴파일러가 관리한다.

Rust가 컴파일 과정에서 이런 허상 참조를 어떻게 막는지 직접 실험해보는 것도 도움이 될 것이다.

파일명: *src/main.rs*

> 아래 코드는 컴파일되지 않는다.
{: .prompt-danger }

```rs
fn main() {
    let reference_to_nothing = dangle();
}

fn dangle() -> &String {
    let s = String::from("hello");

    &s
}
```

에러 메시지는 다음과 같다:

```bash
$ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0106]: missing lifetime specifier
 --> *src/main.rs*:5:16
  |
5 | fn dangle() -> &String {
  |                ^ expected named lifetime parameter
  |
  = help: 이 함수의 반환 타입에는 빌린 값이 포함되어 있지만, 빌릴 수 있는 값이 존재하지 않는다
help: consider using the `'static` lifetime
  |
5 | fn dangle() -> &'static String {
  |                 +++++++

`rustc --explain E0106`을 실행하면 이 에러에 대한 자세한 정보를 얻을 수 있다.
error: `ownership`을 컴파일할 수 없다. 이전 에러 때문이다.
```

아직 설명하지 않은 생명주기에 대한 기능을 이 에러 메시지가 언급하고 있다. 생명주기 부분을 제외하고 본다면, 이 메시지는 코드에 문제가 있는 주된 이유를 담고 있다:

```bash
이 함수의 반환 타입에는 빌린 값이 포함되어 있지만, 빌릴 수 있는 값이 존재하지 않는다
```

`dangle` 함수 내부에서 무슨 일이 일어나는지 살펴보자.

파일명: *src/main.rs*

```rs
fn dangle() -> &String { // dangle은 String에 대한 참조를 반환한다

    let s = String::from("hello"); // s는 새로 생성된 String이다

    &s // 여기서 String s에 대한 참조를 반환한다
} // 이 지점에서 s는 유효 범위를 벗어나고, 메모리에서 해제된다. 이는 위험하다!
```

`s`는 `dangle` 함수 내부에서 만들어졌으므로, 함수가 끝나면 `s`는 자동으로 해제된다. 그런데 우리는 `s`의 참조를 반환하려고 한다. 이는 참조가 사라진 `String`을 가리키게 되는 것이고, Rust는 이런 코드를 허용하지 않는다.

문제를 해결하는 방법은 `String`을 직접 반환하는 것이다:

```rs
fn no_dangle() -> String {
    let s = String::from("hello");

    s
}
```

이 코드는 소유권을 이동시키기 때문에, `String`이 해제되는 일이 없어서 안전하게 작동한다.

---

## **참조의 기본 규칙들**

참조를 사용할 때 기억해야 할 기본적인 규칙들을 다시 살펴보자:

- **한 번**에 하나의 가변 참조나 여러 개의 불변 참조 중 하나만 가질 수 있다. 즉, 데이터를 변경할 수 있는 가변 참조가 있으면 그 시점에는 다른 어떤 참조도 같은 데이터에 있어서는 안 된다. 반면에 데이터를 변경하지 않는 불변 참조는 여러 개 있어도 괜찮다.
- 모든 참조는 항상 유효해야 한다. 이는 참조가 가리키는 데이터가 그 참조가 존재하는 동안에는 항상 존재해야 함을 의미한다.

이제 우리는 참조의 또 다른 형태인 **슬라이스**에 대해 알아볼 차례이다.

---

> 출처: [rust-lang book](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
{: .prompt-info }
