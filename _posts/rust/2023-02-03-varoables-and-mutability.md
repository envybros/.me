---
title: "[Rust] 변수와 가변성"
categories: [Rust 연구소]
tags: [Rust, Variable, Mutability]
date: 2023-02-03 01:30
img_path: /assets/img/rust/
---

---

![Title](rust_title.png)

---

## **개요**

앞서 언급했듯이, 기본적으로 변수들은 불변성을 가진다. 이는 Rust가 코드를 안전하고, 병렬 처리하기 쉽도록 작성할 수 있게끔 우리를 유도하기 위한 방법이다. 또한 우리는 필요에 따라 변수를 가변성을 갖도록 설정할 수도 있다. 이제 Rust가 왜 불변성을 중시하는지, 그리고 때로는 그러한 선택을 하지 않아도 되는지, 그 이유에 대해 살펴보자.

변수가 불변성을 가질 때, 해당 변수명에 처음 값이 할당되면 그 값을 다신 바꿀 수 없다. 이 점을 명확히 하기 위해, 우선 프로젝트 디렉토리에서 `cargo new variables` 명령어를 사용하여 *variables*이라는 이름의 새 *프로젝트*를 생성해보자.

그리고 새로 만든 *variables* 디렉토리 안의 *src/main.rs* 파일을 열어 아래의 코드로 교체해보자.

> 아래 코드는 컴파일되지 않는다.
{: .prompt-danger }

파일명: *src/main.rs*

```rs
fn main() {
    let x = 5;
    println!("x의 값은: {}", x);
    x = 6;
    println!("x의 값은: {}", x);
}
```

이 프로그램을 저장하고 `cargo run`을 사용하여 실행해보자. 불변성 에러에 관한 에러 메시지가 출력되어야 한다:

```bash
$ cargo run
   Compiling variables v0.1.0 (file:///projects/variables)
error[E0384]: cannot assign twice to immutable variable `x`
 --> *src/main.rs*:4:5
  |
2 |     let x = 5;
  |         -
  |         |
  |         first assignment to `x`
  |         help: consider making this binding mutable: `mut x`
3 |     println!("The value of x is: {x}");
4 |     x = 6;
  |     ^^^^^ cannot assign twice to immutable variable

For more information about this error, try `rustc --explain E0384`.
error: could not compile `variables` due to previous error
```

이 예시는 컴파일러가 우리의 프로그램에서 에러를 찾아내는 방법을 보여준다. 컴파일 에러는 골치 아픈 문제일 수 있지만, 이것은 프로그램이 아직 우리가 원하는 대로 안전하게 동작하지 않는다는 것일 뿐, 우리가 능력이 없는 개발자라는 의미는 아니다. 심지어 경험 많은 Rust 개발자조차도 컴파일 에러를 접하게 된다.

불변 변수 `x`에 두 번째 값을 할당하려 한 것이 에러의 원인이기 때문에, **불변 변수 x에 두 번 할당할 수 없다**는 에러 메시지가 나타났다.

불변으로 지정된 값을 변경하려 할 때, 컴파일 시간에 에러를 받게 되는 것은 중요하다. 이러한 상황이 버그를 초래할 수 있기 때문이다. 코드가 어떤 값이 절대 바뀌지 않을 것이라는 가정하에 동작하는 경우, 다른 곳에서 그 값을 변경한다면, 해당 코드의 동작이 예상대로 이루어지지 않을 수 있다. 만약 값이 아주 가끔 변경되는 상황이라면, 이런 종류의 버그는 원인을 추적하기가 매우 어려울 수 있다.

Rust 컴파일러는 값을 변경하지 않을 것이라고 선언하면, 그 값이 정말로 변경되지 않도록 보장한다. 따라서 우리는 그것을 우리 스스로 직접 관리할 필요가 없다. 더 나아가 코드를 추론하기 더 쉽게 할 수 있다.

하지만 가변성도 굉장히 유용하며, 코드 작성을 더욱 편리하게 해준다. 변수는 기본적으로 불변이지만, `mut`을 변수명 앞에 추가함으로써 가변으로 만들 수 있다. `mut`은 이 변수의 값이 코드의 다른 부분에서 변경될 수 있음을 추후 코드를 읽을 사람들에게 명확히 전달한다.

이에 따라, *src/main.rs* 파일을 다음과 같이 변경해 보도록 하자:

파일명: *src/main.rs*

```rs
fn main() {
    let mut x = 5;
    println!("x의 값은: {}", x);
    x = 6;
    println!("x의 값은: {}", x);
}
```

프로그램을 실행하면 다음과 같은 결과가 나타난다:

```bash
$ cargo run
   Compiling variables v0.1.0 (file:///projects/variables)
    Finished dev [unoptimized + debuginfo] target(s) in 0.30s
     Running `target/debug/variables`
x의 값은: 5
x의 값은: 6
```

`mut`을 사용하면 `x`에 바인딩된 값을 `5`에서 `6`으로 변경할 수 있다. 결국 가변성을 사용할지 여부는 개발자가 상황에 따라 판단하고, 결정해야 한다.

---

## **상수**

변수와 유사하게, 상수도 변수명에 바인딩된 값을 가지며, 이 값은 변경할 수 없다. 하지만 상수와 변수 사이에는 몇 가지 차이점이 존재한다.

우선, 상수와 `mut` 키워드를 함께 사용할 수 없다. 상수는 단순히 기본 설정만 불변인 것이 아니라 항상 불변이기 때문이다. 상수는 `let` 키워드가 아닌 `const` 키워드를 사용하여 선언하며, 값의 타입을 반드시 명시해야 한다.

또한, 상수는 글로벌 스코프를 포함하여 어디에서나 선언할 수 있다. 이는 코드의 여러 부분에서 알아야 할 값들에 대해 유용하게 사용될 수 있다.

마지막으로, 상수는 상수 표현식에서만 설정할 수 있다. 즉, 상수는 런타임에 계산되어야 하는 값으로는 설정할 수 없다.

다음은 상수 선언의 예시이다:

```rs
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
```

상수의 이름은 `THREE_HOURS_IN_SECONDS`이며, 값은 분당 60초와 시간당 60분을 곱한 후, 이 프로그램에서 계산하고자 하는 3시간을 다시 곱한 값이다. Rust에서의 상수의 이름 규칙은 모든 글자를 대문자로 하고, 단어 사이에는 밑줄을 넣는 것이다. 컴파일러는 컴파일 시간에 제한된 연산들을 평가할 수 있으므로, 이 값을 10,800으로 직접 설정하기보다는, 계산 과정이 명확하게 드러나는 방식으로 표현하는 것이 더 이해하기 쉽고 검증하기에 용이하다.

상수는 선언된 스코프 내에서 프로그램이 실행되는 동안 계속해서 유효하다. 이러한 특성은 게임의 플레이어가 획득할 수 있는 최대 점수나 빛의 속도 등, 애플리케이션 영역에서 여러 부분이 공유할 수 있는 중요한 정보를 다룰 때 유용하게 사용된다.

프로그램 전반에 걸쳐 사용되는 하드코딩된 값들을 상수로 명명하는 것은, 그 값이 어떤 의미를 가지는지 코드의 유지 보수 담당자에게 명확히 전달하는 데 도움이 된다. 또한, 나중에 이러한 하드코딩된 값이 변경되어야 할 경우, 코드 내에서 변경해야 할 부분이 한 곳뿐이므로 매우 유용하다.

---

## **쉐도잉**

숫자 맞추기 게임에서 본 것처럼, Rust에서는 기존에 선언된 변수와 동일한 이름의 새로운 변수를 선언할 수 있다. 이 경우, 첫 번째 변수는 두 번째 변수에 의해 **쉐도잉**되었다고 말한다. 이는 해당 변수명을 사용할 때 컴파일러가 두 번째 변수를 인식한다는 것을 의미한다. 결과적으로 두 번째 변수가 첫 번째 변수를 가리게 되어, 코드에서 변수명이 언급될 때마다 두 번째 변수만을 가리키게 된다. 이러한 쉐도잉은 `let` 키워드의 반복적인 사용과 동일한 변수명 사용을 통해 이루어진다:

파일명: *src/main.rs*

```rs
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("내부 스코프의 x 값은: {x}");
    }

    println!("x의 값은: {x}");
}
```

이 프로그램은 처음에 `x`라는 이름의 변수에 `5`라는 값을 할당한다. 이어서 같은 이름의 새로운 변수를 선언하여 원래 `x`의 값에 `1`을 더하고, 결과적으로 `x`의 값이 `6`이 되게 한다. 추가적으로 중괄호를 사용하여 생성한 내부 스코프에서, 세 번째 `let` 문은 또다시 `x`를 쉐도잉하며 이전 값에 `2`를 곱해 `x`의 값이 `12`가 되도록 한다. 내부 스코프가 끝나면, 쉐도잉된 `x`는 사라지고 원래의 `x` 값인 `6`이 복원된다. 이 프로그램을 실행하면, 다음과 같은 결과를 확인할 수 있을 것이다:

```bash
$ cargo run
   Compiling variables v0.1.0 (file:///projects/variables)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/variables`
내부 스코프의 x 값은: 12
x의 값은: 6
```

쉐도잉은 변수를 가변(mutable)으로 표시하는 것과는 다르다. `let` 키워드를 사용하지 않고 변수에 재할당하려고 하면 컴파일 시간에 에러가 발생한다. `let`을 사용하면 값에 몇 가지 변형을 가할 수 있지만, 그 변형들이 완료된 후에는 해당 변수가 불변(immutable) 상태가 된다.

또한, `let` 키워드를 다시 사용함으로써 실질적으로 새 변수를 생성하기 때문에, `mut`와 쉐도잉 사이에는 또 다른 차이점이 존재한다. 즉, 변수의 타입을 변경할 수 있지만 이름은 동일하게 유지할 수 있다. 예를 들어, 우리의 프로그램이 사용자로부터 특정 텍스트 간격에 들어갈 공백 수를 입력받아 그 입력값을 숫자로 저장하고자 할 때를 생각해보자:

```rs
    let spaces = "   ";
    let spaces = spaces.len();
```

첫 번째 `spaces` 변수는 문자열 타입이고 두 번째 `spaces` 변수는 숫자 타입이다. 쉐도잉을 사용하면 `spaces_str`과 `spaces_num`과 같은 다른 이름을 사용할 필요 없이 `spaces`라는 동일한 이름을 재사용할 수 있다. 그러나 이 경우 `mut`를 사용하면, 컴파일 시간에 에러가 발생한다.

```rs
    let mut spaces = "   ";
    spaces = spaces.len();
```

에러 메시지는 변수의 타입을 변경할 수 없다고 지적한다:

```bash
$ cargo run
   Compiling variables v0.1.0 (file:///projects/variables)
error[E0308]: mismatched types
 --> *src/main.rs*:3:14
  |
2 |     let mut spaces = "   ";
  |                      ----- expected due to this value
3 |     spaces = spaces.len();
  |              ^^^^^^^^^^^^ expected `&str`, found `usize`

For more information about this error, try `rustc --explain E0308`.
error: could not compile `variables` due to previous error
```

이제 변수가 어떻게 작동하는지 알아보았으니, 변수가 가질 수 있는 다양한 데이터 타입에 대해 자세히 알아보자.

---

> 출처: [rust-lang book](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)
{: .prompt-info }
