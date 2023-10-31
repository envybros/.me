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

대부분의 프로그래밍 언어는 조건이 `true`인 경우 코드를 실행하거나, 조건이 계속 `true`인 동안 코드를 반복해서 실행하는 기능을 제공한다. Rust에서는 `if` 표현식과 루프로 실행 흐름을 제어한다.

## **if 표현식**

`if` 표현식으로 조건에 따른 코드의 실행 여부를 결정할 수 있다. "조건이 참일 때 해당 코드 블록을 실행하며, 조건이 거짓일 때는 실행하지 않는다."로 이해할 수 있다.

projects 디렉토리에 *branches*라는 새 프로젝트를 생성하고, if 표현식을 살펴보자. *src/main.rs* 파일에 다음 내용을 작성해보자.

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

`if` 표현식은 항상 `if` 키워드로 시작하며, 그 뒤에 조건이 따라온다. 예를 들어, 위의 예제에서는 `number` 변수가 5보다 작은지 확인하는 조건을 사용했다. 조건이 `true`일 때 실행되는 코드 블록은 조건 다음 중괄호 내에 위치한다. `if` 표현식의 이러한 코드 블록을 분기(*arms*)라 부른다. 이는 예전에 다뤘던 `match` 표현식의 분기와 관련이 있다.

조건이 `false`일 경우 실행될 코드는 `else` 표현식으로 작성할 수 있다. `else` 표현식을 작성하지 않을 경우, 조건이 `false`이면 `if` 블록을 건너뛰고, 다음 코드로 이동한다.

위 코드를 실행하면, 아래와 같은 결과가 출력된다:

```bash
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31s
     Running `target/debug/branches`
조건은 true이다.
```

변수 `number`의 값을 변경하여 조건이 `false`일 때 어떤 반응이 나타나는지 확인해보자.

```rs
    let number = 7;
```

코드를 변경한 후, 프로그램을 다시 실행하면 다음과 같은 결과를 확인할 수 있다:

```bash
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
    Finished dev [unoptimized + debuginfo] target(s) in 0.31s
     Running 'target/debug/branches'
조건은 false이다.
```

조건문의 조건은 반드시 불리언 타입(`bool`)이어야 한다. 불리언 타입이 아닌 값으로 조건을 설정하면 에러가 발생한다. 아래 코드를 보면 더 잘 이해할 수 있다:

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

위 코드에서는 `if` 조건이 `3`의 값으로 평가되었고, Rust에서는 에러를 반환한다:

```bash
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
error[E0308]: mismatched types
 --> *src/main.rs*:4:8
  |
4 |     if number {
  |        ^^^^^^ expected `bool`, found integer

For more information about this error, try `rustc --explain E0308`.
error: could not compile `branches` due to previous error
```

에러 메시지는 Rust가 `bool` 값을 기대하였지만 정수 값을 받았다는 것을 나타낸다. Ruby나 JavaScript와 같은 언어들은 불리언이 아닌 타입을 자동으로 불리언 값으로 변환하지만, Rust에서는 그렇게 하지 않는다. 따라서 `if` 문에서는 반드시 불리언 값을 조건으로 사용해야 한다. 즉, 숫자가 `0`이 아닐 경우에만 `if` 코드 블록이 실행되게 하려면, `if` 표현식을 아래와 같이 수정해야 한다:

파일명: *src/main.rs*

```rs
fn main() {
    let number = 3;

    if number != 0 {
        println!("number는 0이 아니다.");
    }
}
```

이 코드를 실행하면 `number는 0이 아니다.`가 출력된다.

### **else if를 활용한 다중 조건 처리**

`else if` 표현식을 사용하면 여러 조건을 처리할 수 있다. 아래 예제를 통해 확인해보자:

파일명: *src/main.rs*

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

프로그램은 각 `if` 표현식을 순서대로 검사하고, `true`인 첫 번째 조건의 코드 블록을 실행한다. 여기서 6은 2로 나누어 떨어지지만 `number는 2로 나누어 떨어진다`는 메시지는 출력되지 않는다. 이는 Rust가 `true`인 첫 번째 조건을 만나면 이후 조건들을 확인하지 않기 때문이다.

다수의 `else if` 조건 사용은 코드의 복잡성을 증가시킬 수 있다. 여러 조건들이 존재할 때는 코드를 리팩터링해야 한다. 추후에 Rust가 제공하는 강력한 분기 처리 도구인 `match`에 대해 알아볼 것이다.

---

### **let 문에서 if 사용하기**

`if`는 표현식이므로, `let` 문의 오른쪽에도 사용할 수 있다. 즉, `if` 표현식을 통해 변수에 결과 값을 할당할 수 있다. 아래 코드에서 예시를 확인할 수 있다.

파일명: *src/main.rs*

```rs
// if 표현식의 결과를 변수에 할당하기

fn main() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("number의 값은: {number}이다.");
}
```

위 코드에서 `number` 변수는 `if` 표현식의 결과에 따라 값이 할당된다.

```bash
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
    Finished dev [unoptimized + debuginfo] target(s) in 0.30s
     Running `target/debug/branches`
number의 값은: 5이다.
```

코드 블록은 내부의 마지막 표현식으로 평가되기 때문에, `if` 표현식 전체의 값은 해당 블록의 실행 결과에 따라 결정된다. 이러한 이유로 `if`문의 각 분기 결과는 같은 타입이어야 한다. 타입이 서로 다를 경우, 에러가 발생한다.

> 아래 코드는 컴파일되지 않는다.
{: .prompt-danger }

```rs
fn main() {
    let condition = true;
    let number = if condition { 5 } else { "six" };

    println!("number의 값은: {number}이다.");
}
```

위 코드를 컴파일하려고 시도하면, `if`와 `else`의 반환 타입이 서로 호환되지 않아 에러가 발생한다.

```bash
$ cargo run
   Compiling branches v0.1.0 (file:///projects/branches)
error[E0308]: `if`와 `else`의 타입이 호환되지 않는다.
 --> *src/main.rs*:4:44
  |
4 |     let number = if condition { 5 } else { "six" };
  |                                 -          ^^^^^ 정수를 예상했으나 `&str` 타입 발견
  |                                 |
  |                                 이 부분 때문에 발생한 문제

에러에 대한 자세한 정보는 `rustc --explain E0308`를 통해 확인할 수 있다.
error: 이전 에러로 인해 `branches` 컴파일에 실패했다.
```

`if` 문의 표현식은 정수 값으로, `else` 문의 표현식은 문자열 값으로 평가된다. 변수는 항상 명확한 타입을 가지며, 그 타입은 컴파일 시간에 확정되어야 한다. 변수는 단일 타입을 가져야 하므로 이러한 혼합 타입은 허용되지 않는다.

Rust는 컴파일 시간에 `number` 변수의 타입을 확실히 알아야 한다. `number`의 타입을 통해 컴파일러는 코드 전체에서 해당 타입이 올바르게 사용되었는지 검증한다. 만약 `number`의 타입 결정이 런타임에 이루어진다면, Rust는 그것을 처리하는데 어려움을 겪게 된다. 이 경우, 컴파일러는 더 복잡한 상황에 처하게 되며, 여러 가상 타입을 추적해야 하므로 코드에 대한 보장도 줄어들게 된다.

---

## **반복문을 사용한 반복**

코드 블록을 여러 번 실행할 필요가 있다. 이 작업을 위해 Rust는 반복문을 제공한다. 반복문은 코드의 마지막까지 실행한 다음, 처음부터 다시 시작한다. 반복문에 대해 알아보기 위해 *loops*라는 새 프로젝트를 만들어보자.

Rust에는 총 세 종류의 반복문, `loop`, `while`, `for`을 제공한다. 각각에 대해 알아보자.

### **loop를 이용한 코드의 반복**

`loop` 키워드는 Rust에게 코드 블록을 무한히 반복 실행하도록 지시한다. 이는 개발자가 명시적으로 중지하기 전까지 계속 반복된다.

*loops* 디렉토리 내의 *src/main.rs* 파일을 다음과 같이 변경해보자:

파일명: *src/main.rs*

```rust
fn main() {
    loop {
        println!("again!");
    }
}
```

이 프로그램을 실행하면, 개발자가 프로그램을 수동으로 중지하기 전까지 계속해서 `again!`이 출력된다. 대부분의 터미널은 키보드 단축키 ctrl-c를 사용하여 연속적인 반복문을 멈출 수 있다.

```bash
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
    Finished dev [unoptimized + debuginfo] target(s) in 0.29s
     Running `target/debug/loops`
again!
again!
again!
again!
^Cagain!
```

여기서 `^C`는 ctrl-c를 누른 위치를 나타낸다. ctrl-c 이후에 `again!`이 출력되는지는 중단 명령을 받았을 때의 반복문의 위치에 따라 다를 수 있다.

물론, Rust는 코드를 이용하여 반복문을 중단하는 방법도 제공한다. `break` 키워드를 반복문 내에 넣어서 프로그램에 언제 반복문 실행을 중지할지 알릴 수 있다. 이전에 학습했던 "숫자 맞추기 게임" 섹션에서, 사용자가 정확한 숫자를 맞춰 게임에서 이겼을 때 프로그램을 종료하기 위해 이 방법을 사용했었다.

또한, 우리는 이전에 `continue`를 사용했는데, 우리는 `continue`로 반복문에서 남은 코드를 건너뛰고 다음 반복으로 바로 넘어가도록 프로그램에 지시할 수 있었다.

---

### **loop를 통한 값 반환**

`loop`는 가끔 실패 가능성이 있는 작업을 재시도하기 위해 사용된다(예시: 스레드가 작업을 모두 마쳤는지 확인). 그리고 이러한 작업의 결과를 `loop` 바깥 코드로 전달해야 할 때도 있다. 그럴 때는, `loop`를 멈추는 `break` 표현식 뒤에 반환할 값을 지정하면, 해당 값은 `loop`에서 반환되어 사용될 수 있다. 다음은 그런 방식을 보여주는 예시이다:

```rs
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("결과는: {result}");
}
```

`loop` 시작 전에 `counter`라는 변수를 선언하며 `0`으로 초기화한다. 그 후, `loop`의 반환 값을 저장할 `result` 변수를 선언한다. `loop` 내에서는 `counter` 변수에 `1`을 계속 더하다가 `counter`가 `10`이 되면, `counter * 2` 값과 `break`를 사용해 `loop`를 종료한다. 그 다음에 `result`에 저장된 값을 출력한다. 이 경우 출력 값이 `20`이 되는 것을 확인할 수 있다.

---

### **다중 반복문에서 루프 레이블 활용**

중첩된 반복문이 있을 때, `break`와 `continue`는 기본적으로 가장 내부의 반복문에만 영향을 준다. 만약 특정 외부의 반복문에 대해서 `break`나 `continue`를 적용하고 싶다면, 루프 레이블을 사용할 수 있다. 루프 레이블은 작은 따옴표(`'`)로 시작한다. 아래에 중첩된 반복문을 사용한 예시를 보여준다:

```rs
fn main() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("최종 count = {count}");
}
```

외부 루프에는 `counting_up`이라는 루프 레이블이 붙어있으며, 0에서 2까지 증가한다. 루프 레이블이 없는 내부 루프는 10에서 9까지 감소한다. 레이블을 지정하지 않은 첫 번째 `break`는 내부 루프만을 종료한다. 반면 `break 'counting_up;`는 외부 루프를 종료한다. 실행 결과는 다음과 같다:

```bash
count = 0
remaining = 10
remaining = 9
count = 1
remaining = 10
remaining = 9
count = 2
remaining = 10
최종 count = 2
```

---

### **while을 활용한 조건부 반복문**

프로그램에서는 종종 반복문 내부에서 조건을 검사할 필요가 있다. 조건이 `true`인 경우 반복문이 실행되며, 조건이 `false`이 될 때 반복문이 멈춘다. 이런 동작은 `loop`, `if`, `else`, `break`를 조합하여 구현할 수 있다.

하지만 이러한 조건부 반복 패턴은 Rust에서 자주 사용되기 때문에, Rust는 `while` 루프를 제공한다. 아래 예제에서는 `while` 루프를 사용하여 조건이 `true`인 동안 세 번 반복하고, 각 반복마다 카운트다운한 후, 반복문이 종료되면 메시지를 출력한다.

```rs
// 조건이 true인 동안 코드를 실행하기 위해 while 루프 사용

fn main() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");
}
```

이런 방식으로 구성하면 `loop`, `if`, `else`, `break`를 사용할 때 발생할 수 있는 복잡한 중첩 구조를 피할 수 있으며, 코드의 가독성도 높아진다. 조건이 `true`이면 코드가 실행되고, `false`이면 루프에서 벗어난다.

---

### **for로 컬렉션 반복**

컬렉션의 요소를 반복할 때, `while` 구조를 선택하여 사용할 수 있다. 예를 들면, 아래의 루프는 배열 `a`의 각 요소를 출력한다.

파일명: *src/main.rs*

```rs
// while 루프를 사용하여 컬렉션의 각 요소를 반복

fn main() {
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("이 값은: {}", a[index]);

        index += 1;
    }
}
```

이 코드에서는 배열의 각 요소에 접근하기 위해 인덱스를 사용하여 순회한다. 인덱스는 `0`부터 시작하고, 배열의 마지막 요소에 도달할 때까지 계속 루프를 실행한다(즉, `index < 5` 조건이 `false`가 될 때까지). 이 코드를 실행하면, 배열에 있는 모든 요소들이 출력된다.

```bash
$ cargo run
   Compiling loops v0.1.0 (file:///projects/loops)
    Finished dev [unoptimized + debuginfo] target(s) in 0.32s
     Running `target/debug/loops`
이 값은: 10
이 값은: 20
이 값은: 30
이 값은: 40
이 값은: 50
```

배열의 모든 값을 터미널에 출력해본 결과, 예상대로 다섯 개의 값이 모두 나타났다. 그러나 `index`가 어느 시점에서든 `5`에 도달하면, 루프는 배열의 여섯 번째 값을 찾기 전에 종료된다.

하지만, 이 방식은 에러를 만들기 쉽다. 만약 인덱스의 값이나 조건식에 실수가 있으면 프로그램에서 패닉 상태가 발생할 수 있다. 예를 들어, 배열 `a`의 크기를 네 개의 요소로 줄였을 때, 조건식을 `while index < 4`로 수정하는 것을 잊어버린다면, 코드는 패닉 상태를 일으킬 것이다. 또한 이 방식은 상대적으로 느릴 수 있다. 컴파일러는 루프의 각 반복마다 배열의 범위 내에서 인덱스가 유효한지 확인하기 위해 조건을 검사하기 때문이다.

더 간결하고 안정적인 방법으로는 `for` 루프를 사용하여 컬렉션의 각 요소를 순회하는 것이다. `for` 루프는 앞서 제시된 코드보다 훨씬 직관적으로 작성할 수 있다.

파일명: *src/main.rs*

```rs
// for 루프를 사용하여 컬렉션의 각 요소를 반복하기

fn main() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("이 값은: {element}");
    }
}
```

이 코드를 실행하면 이전의 코드와 동일한 출력 결과를 확인할 수 있다. 무엇보다도, 이 방법은 코드의 안전성을 향상시키며, 배열의 범위를 넘어서거나 항목을 놓칠 가능성을 줄인다.

`for` 루프를 활용하면, 배열의 값이 변경되더라도 다른 부분의 코드를 수정할 필요가 없다. 이러한 장점(안전성과 간결성)으로 인해 `for` 루프는 Rust에서 가장 자주 사용되는 반복문이다.

또한, 특정 횟수만큼 코드를 반복 실행하려는 경우에도 `for` 루프가 권장된다. 예를 들면, 숫자의 범위를 생성하는 표준 라이브러리인 `Range`를 사용할 수 있다.

아래 코드는 `for` 루프와 `rev` 방법을 활용한 카운트다운의 예를 보여준다.

파일명: *src/main.rs*

```rs
fn main() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}
```

이 코드는 더 간결하고 명확하다.

---

## **정리**

---

우리는 지금까지 변수, 스칼라 및 복합 자료형, 함수, 주석, `if` 표현식, 그리고 반복문에 대하여 배웠다. 이번 섹션에서 배운 내용을 활용하여 다음과 같은 프로그램을 만들어 보는 것을 추천한다:

- 화씨와 섭씨 간의 온도를 변환하는 프로그램
- n번째 피보나치 수를 구하는 프로그램
- "The Twelve Days of Christmas" 크리스마스 캐롤의 가사를 출력하는 프로그램, 이때 노래의 반복 부분을 잘 활용해보자.

다음 섹션에서는 다른 프로그래밍 언어에는 흔히 존재하지 않는 **소유권**이라는 개념을 배울 예정이다.

---

> 출처: [rust-lang book](https://doc.rust-lang.org/book/ch03-05-control-flow.html)
{: .prompt-info }
