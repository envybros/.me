---
title: "[Rust] 참조와 빌림"
categories: [Rust 연구소]
tags: [Rust, Control-flow]
date: 2023-03-09 01:30
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

위 튜플 코드의 문제는 calculate_length를 호출한 후에도 String을 사용하려면 호출하는 함수로 String을 반환해야 한다는 점이다. 그 이유는 String이 calculate_length로 이동했기 때문이다. 하지만 String 값의 참조를 제공할 수 있다. 참조는 특정 주소를 따라 해당 주소에 저장된 데이터에 접근할 수 있는 방법이다. 이 데이터는 다른 변수에 의해 소유된다. 포인터와는 다르게 참조는 그 참조의 기간 동안 특정 타입의 유효한 값을 지칭하도록 보장된다.

다음은 소유권을 가져가는 대신 객체에 대한 참조를 매개변수로 갖는 calculate_length 함수를 정의하고 사용하는 방법이다:

파일명: src/main.rs

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

변수 선언 및 함수 반환 값에서 튜플 코드가 모두 사라진 것을 볼 수 있다. 그리고 calculate_length에 &s1을 전달하고 그 정의에서 String 대신 &String을 사용한다. 이 앰퍼샌드는 참조를 나타내며 소유하지 않고도 값을 참조할 수 있다는 것을 의미한다. 아래 그림은 이 개념을 나타낸다.

> ![refandbor00](refandbor00.png)
>
> [그림 1-1] &String s가 String s1을 가리키는 다이어그램
{: .prompt-img }

---

> 참고: &를 사용하는 참조와 반대되는 개념은 역참조로, * 연산자를 통해 수행된다.
{: .prompt-tip }

---

이제 함수 호출에 대해 좀 더 자세히 알아보자:

```rs
let s1 = String::from("hello");

let len = calculate_length(&s1);
```

s1의 값을 소유하지 않고 참조를 생성할 수 있는 &s1 문법을 사용한다. 그러므로 참조가 더 이상 사용되지 않을 때 가리키는 값은 해제되지 않는다.

마찬가지로 함수의 서명에서 매개변수 s의 타입이 참조임을 나타내기 위해 &를 사용한다. 설명을 위한 주석을 추가해 보자:

```rs
fn calculate_length(s: &String) -> usize { // s는 String에 대한 참조이다.
    s.len()
} // 여기에서 s는 범위를 벗어난다.
  // 그러나 s가 참조하는 값은 s가 소유하고 있지 않기 때문에
  // 해제되지 않는다.
```

변수 s의 범위는 일반적인 함수 매개변수의 범위와 동일하다. 그러나 참조가 가리키는 값은 s가 더 이상 사용되지 않을 때 해제되지 않는다. 이는 s에 소유권이 없기 때문이다. 실제 값 대신 참조를 매개변수로 갖는 함수는 소유권을 반환하기 위해 값을 반환할 필요가 없다.

참조를 생성하는 이 행위를 '빌림'이라고 한다. 실제로 어떤 것을 소유하고 있는 사람으로부터 그것을 빌릴 수 있다. 사용이 끝나면 그것을 다시 돌려주어야 한다.

우리가 빌려온 값에 변형을 시도한다면 어떤 결과가 발생할까?

파일명: *src/main.rs*

> 아래 코드는 컴파일되지 않는다.
{: .prompt-danger }

```rs
// 빌려온 값을 수정하려는 시도

fn main() {
    let s = String::from("hello");

    change(&s);
}

fn change(some_string: &String) {
    some_string.push_str(", world");
}
```

이때 발생하는 에러 메시지는 아래와 같다:

```bash
$ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0596]: `*some_string`을 가변으로 빌리려 했으나, `&` 참조 뒤에 있기 때문에 불가능하다.
 --> src/main.rs:8:5
  |
7 | fn change(some_string: &String) {
  |                        ------- help: 이를 가변 참조로 변경하려면 `&mut String`으로 고려하자.
8 |     some_string.push_str(", world");
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `some_string`은 `&` 참조이기 때문에, 참조하는 데이터를 가변으로 빌릴 수 없다.

에러에 대한 추가 정보를 원한다면, rustc --explain E0596를 시도해보자.
error: 이전의 에러로 인해 ownership 컴파일을 할 수 없다.
```

변수가 기본적으로 불변한 것처럼, 참조 역시 불변이다. 따라서 참조를 통해 어떤 값을 수정할 수 없다.

---

## **가변 참조**

위 코드를 수정해서 빌려온 값을 변경할 수 있도록 하는 방법은 가변 참조를 사용하는 것이다.

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

이 코드에서는 s를 변경 가능하게 선언하였다. 그리고 change 함수를 호출할 때 &mut s로 가변 참조를 만들고, 함수의 매개변수 타입을 &mut String으로 변경하여 가변 참조를 받을 수 있게 하였다. 이렇게 하면 change 함수 내에서 값을 수정할 수 있다.

가변 참조에는 한 가지 중요한 제약이 있다. 한 번에 하나의 가변 참조만 허용된다. 즉, 동시에 같은 값을 여러 번 가변 참조할 수 없다.

아래의 코드를 보면 s에 대해 두 개의 가변 참조를 시도하고 있다:

파일명: *src/main.rs*

> 아래 코드는 컴파일되지 않는다.
{: .prompt-danger }

```rs
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s;

    println!("{}, {}", r1, r2);
```

이 코드를 실행하면 다음과 같은 에러 메시지가 출력된다:

```bash
$ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0499]: `s`를 한 번에 두 번 이상 가변으로 빌릴 수 없다.
 --> src/main.rs:5:14
  |
4 |     let r1 = &mut s;
  |              ------ 첫 번째 가변 빌림이 이곳에서 발생한다.
5 |     let r2 = &mut s;
  |              ^^^^^^ 두 번째 가변 빌림이 이곳에서 발생한다.
6 |
7 |     println!("{}, {}", r1, r2);
  |                        -- 첫 번째 빌림은 이후 여기서 사용된다.

에러에 대한 추가 정보를 원한다면, rustc --explain E0499를 시도하라.
error: 이전의 에러로 인해 ownership 컴파일을 할 수 없다.
```

이 에러 메시지는 s 변수에 두 번 가변 참조를 동시에 할 수 없다는 것을 알려준다. 첫 번째 가변 참조는 r1에서 발생하며, 두 번째는 r2에서 발생한다.

한 데이터에 대해 동시에 여러 가변 참조를 만드는 것을 금지하는 이 제약은 변형을 통제된 방식으로 허용한다. 대다수 언어에서는 개발자가 원할 때 언제든지 데이터를 변형할 수 있게 허용하기 때문에 Rust를 처음 접하는 개발자들은 이 제약에 대해 낯설게 느낄 수 있다. 이 제약의 가장 큰 장점은 Rust가 컴파일 시 데이터 경쟁을 미연에 방지한다는 것이다. 데이터 경쟁은 경쟁 상태와 비슷한 현상으로, 아래의 세 가지 조건이 충족될 때 발생한다:

- 두 개 이상의 포인터가 동시에 같은 데이터에 접근한다.
- 포인터 중 적어도 하나는 해당 데이터에 값을 쓰기 위해 사용한다.
- 데이터에 대한 접근을 동기화하는 메커니즘이 없다.

데이터 경쟁은 예측할 수 없는 동작을 초래하며, 런타임 시점에서 이를 파악하고 수정하는 것은 까다롭다. Rust는 컴파일 과정에서 이러한 데이터 경쟁을 포함한 코드를 컴파일하지 않음으로써 이 문제를 해결한다.

그리고 언제나처럼, 중괄호를 이용해 새로운 범위를 생성함으로써 동시에 발생하지 않는 여러 가변 참조를 허용할 수 있다.

```rs
let mut s = String::from('hello');

{
    let r1 = &mut s;
} // r1이 여기에서 범위를 벗어나므로 문제 없이 새로운 참조를 만들 수 있다.

let r2 = &mut s;
```

Rust는 가변 참조와 불변 참조를 결합할 때 유사한 규칙을 적용한다. 이 코드는 에러를 발생시킨다:

> 아래 코드는 컴파일되지 않는다.
{: .prompt-danger }

```rs
let mut s = String::from('hello');

let r1 = &s; // 문제 없다
let r2 = &s; // 문제 없다
let r3 = &mut s; // 큰 문제다

println!("{}, {}, 그리고 {}", r1, r2, r3);
```

여기에 나오는 에러는 다음과 같다:

```bash
$ cargo run
   Compiling ownership v0.1.0 (file:///projects/ownership)
error[E0502]: `s`를 불변으로 빌린 상태에서 가변으로 빌릴 수 없다.
 --> src/main.rs:6:14
  |
4 |     let r1 = &s; // 문제 없다
  |              -- 불변으로 빌림이 이루어진다
5 |     let r2 = &s; // 문제 없다
6 |     let r3 = &mut s; // 큰 문제다
  |              ^^^^^^ 가변으로 빌림이 이루어진다
7 |
8 |     println!("{}, {}, 그리고 {}", r1, r2, r3);
  |                                -- 불변으로 빌림이 나중에 여기서 사용된다

이 에러에 대한 자세한 정보는 `rustc --explain E0502`를 참조하라.
error: `ownership` 컴파일에 실패하였다.
```

동일한 값을 가진 불변 참조가 있을 때 가변 참조를 가질 수 없다.

불변 참조의 사용자들은 그 값이 갑자기 변경되길 원치 않는다! 그러나 여러 불변 참조는 데이터를 읽는 사람만이 데이터의 읽기에 영향을 주는 능력이 없기 때문에 허용된다.

참조의 범위는 그것이 소개되는 곳에서 시작하여 그 참조가 마지막으로 사용되는 시점까지 계속된다. 예를 들어, 이 코드는 불변 참조의 마지막 사용, println!,이 가변 참조가 도입되기 전에 발생하기 때문에 컴파일된다:

```rs
let mut s = String::from('hello');

let r1 = &s; // 문제 없다
let r2 = &s; // 문제 없다
println!("{} 그리고 {}", r1, r2);
// 이 시점 이후 r1과 r2 변수는 사용되지 않는다

let r3 = &mut s; // 문제 없다
println!("{}", r3);
```

불변 참조 r1과 r2의 범위는 가변 참조 r3가 생성되기 전에, 그것들이 마지막으로 사용된 println! 이후에 끝난다. 이 범위들은 겹치지 않으므로 이 코드는 허용된다: 컴파일러는 참조가 더 이상 사용되지 않는 시점이 범위의 끝 전임을 알 수 있다.

빌림에 관한 에러가 때로는 짜증나는 순간이 있을지라도, 이것이 Rust 컴파일러가 잠재적인 버그를 런타임 전인 컴파일 시간에 조기에 지적하고 문제가 정확히 어디에 있는지 보여준다는 것을 기억하라. 그러면 개발자는 왜 데이터가 자신이 생각했던 것과 다른지 추적할 필요가 없다.

---

> 출처: [rust-lang book](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)
{: .prompt-info }
