---
title: "사용자 정의 타입"
categories: [Rust 연구소]
tags: [Rust]
date: 2023-08-07 00:10
---

## **사용자 정의 타입(Custom Types)**

Rust의 사용자 정의 타입은 주로 두 개의 키워드를 통해 구성된다.

- `struct`: 구조체를 정의한다.
- `enum`: 열거형을 정의한다.

상수는 `const` 및 `staic` 키워드를 통해 생성할 수 있다.

---

## **구조체(Structures)**

`struct` 키워드를 사용하여 만들 수 있는 구조체("structs")에는 세 가지 타입이 있다.

- 튜플 구조체: 기본적으로, 튜플로 명명된다.
- C 구조체: 고전적인 [C스타일](https://en.wikipedia.org/wiki/Struct_(C_programming_language))의 구조체
- 단위 구조체: 필드가 없는 단위 구조체는, 제너릭에 꽤 유용하다.

```rust
// 사용하지 않는 코드에 대한 경고를 숨겨주는 어트리뷰트
#![allow(dead_code)]

#[derive(Debug)]
struct Person {
    name: String,
    age: u8,
}

// 단위 구조체
struct Unit;

// 튜플 구조체
struct Pair(i32, f32);

// 2개의 필드를 가진 구조체
struct Point {
    x: f32,
    y: f32,
}

// 구조체를 다른 구조체의 필드로 재사용할 수 있다.
struct Rectangle {
    // Rectangle은 왼쪽 상단과 오른쪽 하단 모서리의 위치로 지정할 수 있다.
    top_left: Point,
    bottom_right: Point,
}

fn main() {
    // 필드 초기화 단축키를 사용하여 구조체 생성
    let name = String::from("Peter");
    let age = 27;
    let peter = Person { name, age };

    // 구조체 디버그 출력
    println!("{:?}", peter);

    // `Point` 인스턴스화
    let point: Point = Point { x: 10.3, y: 0.4 };

    // `Point`의 필드에 액세스
    println!("point coordinates: ({}, {})", point.x, point.y);

    // 구조체 업데이트 구문을 사용하여 다른 구조체의 필드를
    // 통해 새로운 `Point`를 만든다.
    let bottom_right = Point { x: 5.2, ..point };

    // `bottom_right.y`는 `point`에서 해당 필드를 사용했기에,
    // `point.y`와 동일하다.
    println!("second point: ({}, {})", bottom_right.x, bottom_right.y);

    // `let` 바인딩을 사용하여 `Point`의 구조를 변경한다.
    let Point { x: left_edge, y: top_edge } = point;

    let _rectangle = Rectangle {
        // 구조체 인스턴스화도 표현식이다.
        top_left: Point { x: left_edge, y: top_edge },
        bottom_right: bottom_right,
    };

    // 단위 구조체 인스턴스화
    let _unit = Unit;

    // 튜플 구조체 인스턴스화
    let pair = Pair(1, 0.1);

    // 튜플 구조체의 필드에 액세스
    println!("pair contains {:?} and {:?}", pair.0, pair.1);

    // 튜플 구조체 해제 (Destructure)
    let Pair(integer, decimal) = pair;

    println!("pair contains {:?} and {:?}", integer, decimal);
}
```

### **활동**

1. `Rectangle`의 면적을 계산하는 함수 `rect_area`를 추가해보자. (nested destructing을 사용해보자.)
2. `Point`와 `f32`를 인자로 받고, `Point`의 왼쪽 상단 모서리와 `f32`에 해당하는 width와 height가 있는 `Rectangle`을 반환하는 함수 `square`를 추가해보자.

### **추가로 읽어보기:**

[attributes](https://doc.rust-lang.org/rust-by-example/attribute.html), [destructuring](https://doc.rust-lang.org/rust-by-example/flow_control/match/destructuring.html)

---

## **열거형(Enums)**

`enum` 키워드를 사용하면 여러 가지 변형 중 하나가 될 수 있는 타입을 만들 수 있다. `struct`로서 가능한 모든 변형은 `enum`에서도 유효하다.

```rust
// Create an `enum` to classify a web event. Note how both names and type information together specify the variant:
// `PageLoad != PageUnload` and `KeyPress(char) != Paste(String)`.
// Each is different and independent.
enum WebEvent {
    // An `enum` variant may either be `unit-like`,
    PageLoad,
    PageUnload,
    // like tuple structs,
    KeyPress(char),
    Paste(String),
    // or c-like structures.
    Click { x: i64, y: i64 },
}

// A function which takes a `WebEvent` enum as an argument and
// returns nothing.
fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unloaded"),
        // Destructure `c` from inside the `enum` variant.
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        // Destructure `Click` into `x` and `y`.
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        },
    }
}

fn main() {
    let pressed = WebEvent::KeyPress('x');
    // `to_owned()` creates an owned `String` from a string slice.
    let pasted  = WebEvent::Paste("my text".to_owned());
    let click   = WebEvent::Click { x: 20, y: 80 };
    let load    = WebEvent::PageLoad;
    let unload  = WebEvent::PageUnload;

    inspect(pressed);
    inspect(pasted);
    inspect(click);
    inspect(load);
    inspect(unload);
}
```


---

참고 링크: [Custom Types](https://doc.rust-lang.org/rust-by-example/custom_types.html)
