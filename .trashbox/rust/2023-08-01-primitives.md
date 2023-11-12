---
title: "기본 요소"
categories: [Rust 연구소]
tags: [Rust]
date: 2023-08-01 00:10
---

## **기본 요소(Primitives)**

Rust에서는 다양한 `기본 요소(Primitives)`에 대한 액세스를 제공한다. 예를 들면:

### **스칼라(Scalar) 타입**

- 부호 있는 정수(Signed integers): `i8`, `i16`, `i32`, `i64`, `i128`, 그리고 `isize`(포인터 사이즈)
- 부호 없는 정수(Unsigned integers): `u8`, `u16`, `u32`, `u64`, `u128`, 그리고 `usize`(포인터 사이즈)
- `char`: `'a'`, `'α'`, `'∞'`와 같은 유니코드 스칼라 값(각각 4bytes)
- `bool`: `true`나 `false`
- 단일 타입 `()`: 가능한 유일한 값이 빈 튜플 하나: `()`.

위 단일 타입의 값은 튜플이다. 하지만 여러 값을 포함하지 않으므로 복합 타입으로 간주되지 않는다.

### **복합(Compound) 타입**

- 배열: `[1, 2, 3]`
- 튜플: `(1, true)`

*변수*는 *타입 어노테이션*을 붙일 수 있다. 숫자는 *접미사* 또는 *기본값*을 통해 어노테이션을 붙일 수 있다. 여기서 정수(integer)는 기본값이 `i32`이고 부동 소수점(float)은 `f64`이다. Rust는 문맥(Context)을 통해 타입을 유추할 수도 있다.

```rust
fn main() {
    // 변수에는 타입 어노테이션을 붙일 수 있다.
    let logical: bool = true;

    let a_float: f64 = 1.0;  // 일반 어노테이션
    let an_integer   = 5i32; // 접미사 어노테이션

    // 또는 기본값이 사용된다.
    let default_float   = 3.0; // `f64`
    let default_integer = 7;   // `i32`

    // 문맥(Context)을 통해 타입을 유추할 수도 있다.
    let mut inferred_type = 12; // i64 타입은 다른 줄에서 유추된다.
    inferred_type = 4294967296i64;

    // 가변(mutable) 변수의 값을 변경할 수 있다.
    let mut mutable = 12; // Mutable `i32`
    mutable = 21;

    // Error! 변수의 타입은 변경할 수 없다.
    mutable = true;

    // 변수는 아래처럼 섀도잉으로 덮어쓸 수 있다.
    let mutable = true;
}
```

### **추가로 읽어보기:**

[std 라이브러리](https://doc.rust-lang.org/std/), [mut](https://doc.rust-lang.org/stable/rust-by-example/variable_bindings/mut.html), [interface](https://doc.rust-lang.org/stable/rust-by-example/types/inference.html), [shadowing](https://doc.rust-lang.org/stable/rust-by-example/variable_bindings/scope.html)

---

## **리터럴 및 연산자**

Integer `1`, Float `1.2`, Character `'a'`, String `"abc"`, Boolean `true` 그리고 단위 타입 `()`은 전부 리터럴을 사용하여 표현할 수 있다.

정수는 이러한 접두사를 사용하여 16진수, 8진수, 또는 2진수 표기법을 표현할 수 있다: `0x`, `0o` 또는 `0b`.

가독성을 높이기 위해, 숫자 리터럴에 밑줄을 삽입할 수도 있다. 예를 들어, `1_000`은 `1000`과 같고, `0.000_001`은 `0.000001`과 같다.

Rust는 과학 표기법인 [E-notation](https://en.wikipedia.org/wiki/Scientific_notation#E_notation)도 지원한다. 예를 들면 `1e6`, `7.6e-4`가 있다. 관련 타입은 `f64`이다.

우리는 컴파일러에 우리가 사용할 리터럴의 타입을 알려줘야 한다. 지금은 리터럴이 부호 없는 32-bit 정수임을 나타내기 위해 `u32` 접미사를 사용하고, 부호 있는 32-bit 정수임을 나타내기 위해 `i32` 접미사를 사용해볼 것이다.

[Rust](https://doc.rust-lang.org/reference/expressions.html#expression-precedence)에서 사용할 수 있는 연산자와 그 우선 순위는 다른 [C 계열 언어](https://en.wikipedia.org/wiki/Order_of_operations#Programming_languages)와 유사하다.

```rust
fn main() {
    // 정수 덧셈
    println!("1 + 2 = {}", 1u32 + 2);

    // 정수 뺄셈
    println!("1 - 2 = {}", 1i32 - 2);
    // TODO ㄴ `1i32`를 `1u32`로 변경해보고, 타입이 왜 중요한지 알아보자

    // 과학적 표기법
    println!("1e4 is {}, -2.5e-3 is {}", 1e4, -2.5e-3);

    // bool 로직 (이산 수학 스타일)
    println!("true AND false is {}", true && false);
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

    // 비트 단위 연산
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011u32 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011u32 ^ 0b0101);
    println!("1 << 5 is {}", 1u32 << 5);
    println!("0x80 >> 2 is 0x{:x}", 0x80u32 >> 2);

    // 밑줄을 사용한 가독성 향상 효과
    println!("One million is written as {}", 1_000_000u32);
}
```

---

## **튜플(Tuples)**

튜플은 서로 다른 타입으로 이루어진 값들의 컬렉션이다. 튜플은 괄호`()`로 구성되며, 각 튜플 자체는 타입 서명 `(T1, T2, ...)`을 가진 값으로, `T1`, `T2`는 해당 멤버의 타입이다. 함수는 튜플을 사용하여 여러 값을 반환할 수 있다. 이는 튜플이 원하는 수의 값을 보유할 수 있기 때문이다.

```rust
// 튜플은 함수 인수와 반환 값으로 사용할 수 있다.
fn reverse(pair: (i32, bool)) -> (bool, i32) {
    // `let`은 튜플의 멤버를 변수에 바인딩하는데 사용할 수 있다.
    let (int_param, bool_param) = pair;

    (bool_param, int_param)
}

// 아래 구조체는.. 뒤 "활동"을 위한 구조체이다.
#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn main() {
    // 다양한 타입이 포함된 튜플
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                      -1i8, -2i16, -3i32, -4i64,
                      0.1f32, 0.2f64,
                      'a', true);

    // 튜플 인덱싱을 사용하여 튜플에서 값을 추출할 수 있다.
    println!("Long tuple first value: {}", long_tuple.0);
    println!("Long tuple second value: {}", long_tuple.1);

    // 튜플은 튜플의 멤버가 될 수 있다.
    let tuple_of_tuples = ((1u8, 2u16, 2u32), (4u64, -1i8), -2i16);

    // 또한 튜플은 출력이 가능하다. ({:?})
    println!("tuple of tuples: {:?}", tuple_of_tuples);

    // 하지만 아주 긴 튜플(12개 이상의 요소가 포함된)은 출력 안된다.
    //let too_long_tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13);
    //println!("Too long tuple: {:?}", too_long_tuple);
    // TODO ㄴ 위 두 줄의 주석을 해제해보고.. 컴파일 오류 메시지를 확인해보자.

    let pair = (1, true);
    println!("Pair is {:?}", pair);

    println!("The reversed pair is {:?}", reverse(pair));

    // 단 한 개의 요소만을 가진 튜플을 만들기 위해선 쉼표가 필요하다.
    // 그렇지 않으면 리터럴과 구분이 가질 않는다.
    println!("One element tuple: {:?}", (5u32,));
    println!("Just an integer: {:?}", (5u32));

    // 튜플을 파괴하여 바인딩을 만들 수도 있다.
    let tuple = (1, "hello", 4.5, true);

    let (a, b, c, d) = tuple;
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);
}
```

### **활동**

1. *요약*: 위 예제에서 `Matrix` 구조체에 `fmt::Display` 특성(trait)을 추가하여 디버그 형식 `{:?}` 출력에서 디스플레이 형식 `{}`으로 전환하여 다음과 같이 출력되도록 해보자:

```text
( 1.1 1.2 )
( 2.1 2.2 )
```

[출력 디스플레이](https://www.envys.me/posts/hello-world#디스플레이)에 대한 예제는 여기서 다시 볼 수 있다.

{:start="2"}
2. 행렬을 인수로 받아오고, 두 요소가 바뀐 행렬을 반환하는 리버스 함수를 템플릿으로 사용하는 전치 함수를 추가해보자. 예를 들어:

```rust
println!("Matrix:\n{}", matrix);
println!("Transpose:\n{}", transpose(matrix));
```

결과 출력:

```text
Matrix:
( 1.1 1.2 )
( 2.1 2.2 )
Transpose:
( 1.1 2.1 )
( 1.2 2.2 )
```

---

## **배열과 슬라이스**

배열은 연속된 메모리에 저장된 동일한 타입의 객체 `T`의 집합이다. 배열은 괄호 `[]`를 사용하여 생성되며, 컴파일 시점에 알 수 있는 길이는 타입 서명 `[T; length]`의 일부이다.

슬라이스는 배열과 유사하지만, 컴파일 타임에서는 그 길이를 알 수 없다. 대신 슬라이스는 두 단어로 구성된 객체로, 첫 번째 단어는 데이터에 대한 포인터, 그리고 두 번째 단어는 슬라이스의 길이이다. 단어의 크기는 `usize`와 동일하다. (예: x86-64의 경우 64 bit). 슬라이스는 배열의 섹션을 빌리는 데 사용할 수 있으며, 타입 서명 `&[T]`를 갖는다.

```rust
use std::mem;

// 이 함수는 슬라이스를 빌린다.
fn analyze_slice(slice: &[i32]) {
    println!("First element of the slice: {}", slice[0]);
    println!("The slice has {} elements", slice.len());
}

fn main() {
    // 고정 크기 배열 (타입 서명은 필요하지 않다.)
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    // 모든 타입을 동일한 값으로 초기화할 수 있다.
    let ys: [i32; 500] = [0; 500];

    // 인덱스는 0부터 시작한다!
    println!("First element of the array: {}", xs[0]);
    println!("Second element of the array: {}", xs[1]);

    // `len`은 배열의 요소 수를 반환한다.
    println!("Number of elements in array: {}", xs.len());

    // 배열은 스택 공간에 할당된다.
    println!("Array occupies {} bytes", mem::size_of_val(&xs));

    // 배열은 슬라이스를 통해 자동으로 빌릴 수 있다.
    println!("Borrow the whole array as a slice.");
    analyze_slice(&xs);

    // 슬라이스는 배열의 섹션을 가리킬 수 있다.
    // [starting_index..ending_index] <- 이런 형태이다.
    // `starting_index`는 슬라이스의 첫 번째 인덱스이다..
    // `ending_index`는 슬라이스의 마지막 위치보다 하나 더 많은 숫자.
    println!("Borrow a section of the array as a slice.");
    analyze_slice(&ys[1 .. 4]);

    // 빈 슬라이스 `&[]`의 예:
    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]); // 동일하지만, 복잡

    // 배열은 `Option`을 반환하는 `.get`을 사용하여 안전하게 접근할 수 있다.
    // 아래와 같이 match시킬 수 있고, 멋진 메시지와 함께 프로그램이 종료되길
    // 원한다면, `.expect()`와 함께 사용할 수도 있다.
    // (행복하게 (멍청하게) 계속 진행되는 것보다 훨씬 낫다.)
    for i in 0..xs.len() + 1 { // 이런, 요소가 하나 더 있다!!
        match xs.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("Slow down! {} is too far!", i),
        }
    }

    // 배열에 바인딩되지 않은 인데깅으로 인해 컴파일 타임 오류가 발생한다.
    //println!("{}", xs[5]);
    // 슬라이스에서 바운드 외 인덱싱으로 인해 런타임 오류가 발생한다.
    //println!("{}", xs[..][5]);
}
```

---

참고 링크: [Primitives](https://doc.rust-lang.org/stable/rust-by-example/primitives.html)
