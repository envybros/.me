---
title: "Hello World"
categories: [Rust 연구소]
tags: [Rust]
date: 2023-07-27 00:10
---

## **Hello World**

전통적인 Hello World부터 시작해보자.

```rust
fn main() {
    // 컴파일된 바이너리가 호출될 때 여기부터 실행된다.

    // 콘솔에 텍스트를 출력한다.
    println!("Hello World!");
}
```

`println!`은 콘솔에 텍스트를 출력할 수 있는 "*macro*"이다.

여기서 Rust 컴파일러: `rustc`를 사용하여 바이너리(bin)를 생성할 수 있다.

```bash
rustc hello.rs
```

`rustc`는 실행가능한 `hello` 바이너리를 생성한다.

```bash
./hello
Hello World
```

### **활동**

> `println!`매크로를 활용하여 아래와 같이 출력될 수 있도록 직접 구현해보자.
{: .prompt-tip }

```text
Hello World!
I am a Rustacean!
```

---

## **주석(Comments)**

Rust에는 아래와 같은 주석들을 지원한다:

- 컴파일러에서 무시되는 *일반 주석*:
  - `// 라인 주석: 줄 끝까지 무시할 수 있다.`
  - `/* 블록 주석: 닫는 구분 기호가 보일 때 까지 무시할 수 있다. */`
- HTML 라이브러리 문서로 파싱되는 *문서 주석*:
  - `/// 아래 나올 내용에 대한 라이브러리 문서(Docs)를 생성한다.`
  - `//! 닫는 구분 기호가 나올 때 까지의 라이브러리 문서를 생성한다.`

코드로 알아보자.

```rust
fn main() {
    // 라인 주석의 예시.
    // 줄의 시작 부분에 슬래시가 두 개 있다.
    // 그리고 그 뒤에 쓰여진 내용은 컴파일러가 읽지 않는다.

    // println!("Hello, world!");

    // 위는 실행되지 않았다. 
    // 이제 위 슬래시 두 개를 삭제하고 다시 실행해 보자.

    /*
     * 이것은 주석의 또 다른 유형인 블록 주석이다. 일반적으로
     * 라인 주석이 권장되는 주석 스타일이다. 하지만 블록 주석
     * 은 코드 청크를 일시적으로 비활성화하는데 매우 유용하다.
     * /* 블록 주석은 /* 중첩할 수 있다 */ */ 따라서 몇 번의 키 입력만으로
     * 몇 번의 키 입력만으로 이 main() 함수의 모든 내용을 주석 처리할 수 있다.
     * /*/*/* 직접 주석을 만들어보자! */*/*/
     */

    /*
    참고: 앞의 `*` 열은 전적으로 스타일을 위한 것이므로, 실제로는 똑같이 만들 필요 없다.
    */

    // 라인 주석보다 블록 주석을 사용하면 표현식을 더 쉽게 조작할 수 있다.
    // 주석 구분 기호를 삭제하여 결과를 변경해 보자:
    let x = 5 + /* 90 + */ 5;
    println!("`x` 10인가 100인가? x = {}", x);
}
```

---

## **출력 서식 지정(Formatted print)**

출력은 `std::fmt`에 정의된 `macros`에 의해 처리된다. 그 중 일부는 다음 내용이 포함된다:

- `format!`: 포맷이 지정된(Formatted) 텍스트를 `String`에 쓴다.
- `print!`: `format!`과 동일하지만, 텍스트가 콘솔에 출력된다. (io::stdout)
- `println!`: `print!`와 동일하지만, 뒤에 개행이 추가된다.
- `eprint!`: `print!`와 동일하지만, 텍스트가 표준 에러(io::stderr)로 출력된다.
- `eprintln!`: `eprint!`와 동일하지만, 개행이 추가된다.

모두 동일한 방식으로 텍스트를 구문 분석(Parse)한다. 추가로, Rust에서는 컴파일 타임에 이러한 포맷 유효성를 검사한다.

```rust
fn main() {
    // 일반적으로, `{}`는 모든 인수로서 자동 대체된다. 
    // 그리고, 이는 문자열화된다.
    println!("{} days", 31);

    // 위치 인수를 사용할 수 있다. 
    // `{}` 안에 정수를 지정하면, 해당 인덱스가 결정된다.
    // 인수는 서식 지정 문자열 바로 뒤에서 0부터 시작한다.
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // 정수 뿐 아니라, 아래와 같이 이름을 키로 써도 상관 없다.
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    // `:` 뒤에 서식 지정 문자를 지정하여 다른 서식으로 호출할 수도 있다.
    println!("Base 10:               {}",   69420); // 69420
    println!("Base 2 (binary):       {:b}", 69420); // 10000111100101100
    println!("Base 8 (octal):        {:o}", 69420); // 207454
    println!("Base 16 (hexadecimal): {:x}", 69420); // 10f2c
    println!("Base 16 (hexadecimal): {:X}", 69420); // 10F2C

    // 지정된 너비로 텍스트를 오른쪽 정렬할 수 있다.
    // 이렇게 하면 "1"이 출력된다. (공백 4개 + "1", 그러므로 총 너비: 5)
    println!("{number:>5}", number=1);

    // 0을 추가하여 숫자를 채우고,
    println!("{number:0>5}", number=1); // 00001
    // 기호를 뒤집어 왼쪽으로 조정할 수 있다. 이렇게 하면 "10000"이 출력된다.
    println!("{number:0<5}", number=1); // 10000

    // 포맷 지정자에 `$`를 추가하여 명명된 인수를 사용할 수 있다.
    println!("{number:0>width$}", number=1, width=5);

    // 심지어 Rust는 인수의 개수가 제대로 사용되었는지 확인한다.
    println!("My name is {0}, {1} {0}", "Bond");
    // FIX: ㄴ 위 예제에 "Annie"와 같은 이름을 하나 추가해보자!

    // fmt::Display를 구현하는 유형만 `{}`로 서식을 지정할 수 있다.
    // 사용자 정의 유형은 기본적으로 fmt::Display를 구현하지 않는다.
    #[allow(dead_code)] // 사용하지 않는 모듈에 대해 경고하는 'dead_code'를 비활성화한다.
    struct Structure(i32);

    // `Structure`가 fmt::Display를 구현하지 않기 때문에 컴파일되지 않는다.
    // println!("This struct `{}` won't print...", Structure(3));
    // TODO: ㄴ 이 줄의 주석을 해제해 보자.

    // Rust 1.58 이상에서는 주변 변수에서 인수를 직접 캡처할 수 있다.
    // 위와 마찬가지로 다음과 같이 출력된다: 
    // "    1", 공백 4개 + "1"
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");
}
```

`std::fmt`에는 텍스트 표기를 관리하는 많은 특성(Traits)들이 포함되어 있다. 그 중 중요한 두 가지 기본 형식(Form)은 다음과 같다.

- `fmt::Debug`: `{:?}` 마커를 사용한다. 디버깅을 위한 텍스트 포맷을 지정할 수 있다.
- `fmt::Display`: `{}` 마커를 사용한다. 보다 더 우아하고, 사용자 친화적인 방식으로 텍스트의 형식을 지정할 수 있다.

본 예제에서는, std 라이브러리가 해당 타입들에 대한 구현을 제공하기 때문에, `fmt::Display`를 사용했다. 만약 사용자 정의 타입을 사용하고자 한다면, 더 많은 설정이 필요하다.

`fmt::Display` 특성(Trait)을 구현하면 해당 타입을 `String`으로 [변환](https://doc.rust-lang.org/stable/rust-by-example/conversion/string.html)할 수 있는 `ToString` 특성(Trait)이 자동으로 구현된다.

*46줄*에서의 `#[allow(dead_code)]`는 바로 뒤에 오는 모듈에서만 적용되는 [속성](https://doc.rust-lang.org/stable/rust-by-example/attribute.html)이다.

### **활동**

- 위 코드의 문제를 수정하여 오류 없이 실행되도록 해보자. (FIX 참조)
- Structure struct의 포맷을 시도하려는 줄의 주석을 없애보자. (TODO 참조)
- 출력하려는 `println!` 매크로를 추가해보자: 표시되는 소수점 이하 자릿수를 조절하면 PI가 대략 3.152가 된다. 이 활동의 목적상, PI의 추정치로 `let pi = 3.141592`를 사용한다. (힌트: 표시할 소수점 이하 자릿수 설정에 대해서는 `std::fmt` 문서를 참조하자.)

---

## **디버그**

`std::fmt` 서식 지정 `traits`을 사용하려는 모든 타입은 출력할 수 있는 구현(Implementation)이 필요하다. 자동 구현은 `std` 라이브러리와 같은 타입들에 대해서만 제공된다. 다른 모든 타입들은 어떻게든 수동으로 구현해야 한다.

`fmt::Debug` `traits`는 이를 매우 간단하게 만들 수 있다. 모든 타입은 `fmt::Debug` 구현을 `derive`(자동 생성)할 수 있다. 하지만 수동으로 구현해야 하는 `fmt::Display`는 그렇지가 않다.

```rust
// 이 구조체는 `fmt::Display` 또는 `fmt::Debug`와 함께 출력할 수 없다.
struct UnPrintable(i32);

// `derive` 어트리뷰트는 이 `구조체`를 `fmt::Debug`로 
// 출력할 수 있게 만드는 데 필요한 구현을 자동으로 생성한다.
#[derive(Debug)]
struct DebugPrintable(i32);
```

모든 `std` 라이브러리 타입은 `{:?}`로도 자동으로 출력할 수 있다:

```rust
// `구조체`에 대한 `fmt::Debug` 구현을 도출(Derive)한다.
// 아래 `구조체`는 하나의 `i32`를 포함하는 구조체이다.
#[derive(Debug)]
struct Structure(i32);

// 구조체 `Deep` 안에 `구조체`를 넣습니다.
// 이 또한 출력 가능하게 만들 수 있다.
#[derive(Debug)]
struct Deep(Structure);

fn main() {
    // `{:?}`로 출력하는 것은 `{}`로 출력하는 것과 유사하다.
    println!("{:?} months in a year.", 12);
    println!("{1:?} {0:?} is the {actor:?} name.",
             "Slater",
             "Christian",
             actor="actor's");

    // `구조체`는 출력 가능하다!!
    println!("Now {:?} will print!", Structure(3));

    // `derive`의 문제점은 결과의 모양을 제어할 수 없다는 것이다.
    // 여기서 그냥 `7`만 표시되도록 하려면 어떻게 해야 할까?
    println!("Now {:?} will print!", Deep(Structure(7)));
}
```

`fmt::Display`를 수동으로 구현하여 디스플레이를 제어할 수 있다.

---

## **디스플레이**

`fmt::Debug`는 간결하지 않고, 또 깔끔하게 보이지 않는다. 그래서 출력 모양을 사용자 지정하여 사용하는 것이 유리한 경우가 많다. 이 작업은 `{}` 출력 마커를 사용하는 `fmt::Display`를 수동으로 구현하여 수행해야 한다. 구현은 다음과 같다:

```rust
// `use`를 통해 `fmt` 모듈을 가져와서 이를 사용할 수 있도록 하자.
use std::fmt;

// `fmt::Display`가 구현될 구조체를 정의한다.
// 이는 `i32`를 포함하는 `Structure`라는 이름의 튜플 구조체이다.
struct Structure(i32);

// `{}` 마커를 사용하려면 타입에 대해 
// `fmt::Display` 특성(trait)을 수동으로 구현(impl)해야 한다.
impl fmt::Display for Structure {
    // 이 특성(trait)에는 정확한 서명을 가진 `fmt`가 필요하다.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 제공된 출력 스트림의 첫 번째 요소인
        // `f`를 엄격하게 사용한다.
        // 작업의 성공 유무를 파악하기 위해 `fmt::Result`를 반환한다.
        // `write!`는 `println!`과 매우 유사한 구문을 
        // 사용한다는 점에 주의하자.
        write!(f, "{}", self.0)
    }
}
```

`fmt::Display`는 `fmt::Debug`보다 깔끔할 수는 있지만, 이는 `std` 라이브러리에 문제가 될 수 있다. 만약 모호한 타입은 어떻게 표시해야 할까? 예를 들어, `std` 라이브러리가 모든 `Vec<T>`에 대해 단일 스타일을 구현했다면, 이는 어떤 스타일이 되어야 할까? 여기서 우리는 다음 두 가지 중 하나라 예상해볼 수 있을까?

- `Vec<path>`: `/:/etc:/home/username:/bin`(`:`으로 나눔)
- `Vec<number>`: `1, 2, 3` (`,`로 나눔)

아쉽게도 그렇진 않다. 모든 타입에 대한 이상적인 스타일은 없다. 왜냐하면 `std` 라이브러리에서는 이를 명시하지 않기 때문이다. `fmt::Display`는 `Vec<T>`나 다른 일반 컨테이너에 대해 구현되지 않는다. 따라서 이런 일반적인 케이스에서는 `fmt::Debug`를 사용해야 한다.

하지만 일반적이지 않은 새로운 컨테이너 타입에 대해서는 `fmt::Display`를 구현할 수 있기 때문에 별 문제 되진 않는다.

```rust
use std::fmt; // `fmt` 가져오기

// 숫자 두 개를 담고 있는 구조체.
// `Debug`가 파생(Derive)되어 `Display`와 결과를 비교할 수 있다.
#[derive(Debug)]
struct MinMax(i64, i64);

// `MinMax`에 대한 `Display`를 구현한다.
impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // `self.number`를 사용하여 각 데이터 포인트를 참조한다.
        write!(f, "({}, {})", self.0, self.1)
    }
}

// 비교를 위해, 필드에 이름을 지정할 수 있는 구조체를 정의해보자.
#[derive(Debug)]
struct Point2D {
    x: f64,
    y: f64,
}

// 마찬가지로, `Point2D`에 대한 `Display`를 구현한다..
impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // `x`와 `y`만 표시될 수 있도록 커스터마이징 해보자.
        write!(f, "x: {}, y: {}", self.x, self.y)
    }
}

fn main() {
    let minmax = MinMax(0, 14);

    println!("Compare structures:");
    println!("Display: {}", minmax);
    println!("Debug: {:?}", minmax);

    let big_range =   MinMax(-300, 300);
    let small_range = MinMax(-3, 3);

    println!("The big range is {big} and the small is {small}",
             small = small_range,
             big = big_range);

    let point = Point2D { x: 3.3, y: 7.2 };

    println!("Compare points:");
    println!("Display: {}", point);
    println!("Debug: {:?}", point);

    // Error. `Debug`와 `Display`가 모두 구현되었지만,
    // `{:b}`를 구현하기 위해선 `fmt::Binary`가 필요하다.
    // 그러므로 이는 제대로 작동하지 않는다.
    // println!("What does Point2D look like in binary: {:b}?", point);
}
```

따라서 `fmt::Display`는 구현되었지만, `fmt::Binary`는 구현되지 않았으므로, 사용할 수 없다. `std::fmt`에는 이러한 특성(trait)들이 많으며 모두 자체 구현이 필요하다. 이에 대한 자세한 내용은 [std::fmt](https://doc.rust-lang.org/std/fmt/)에서 확인할 수 있다.

### **활동**

위 예제의 출력을 확인해보고, `Point2D` 구조체를 가이드 삼아 예제에 `Complex` 구조체를 추가해보자. 결과는 다음과 같이 출력되어야 한다.

```text
Display: 3.3 + 7.2i
Debug: Complex { real: 3.3, imag: 7.2 }
```

---

## **Testcase: List**

요소를 각각 순차적으로 처리해야 하는 구조에 대해, `fmt::Display`를 사용하여 구현하는 것은 꽤 복잡할 수 있다. 여기서 문제는, 각 `write!`가 `fmt::Result`를 생성한다는 것이다. 이를 적절히 처리하기 위해서는 모든 결과를 처리해야 한다. 바로 이 목적을 위해 Rust에서는 `?`연산자를 제공한다.

`write!`에 `?`를 사용한 예제는 다음과 같다:

```rust
// `write!`를 시도하여, 오류가 발생하는지 확인한다.
// 만약 오류가 발생하면, 해당 오류를 반환한다. 그렇지 않았다면, 계속 진행한다.
write!(f, "{}", value)?;
```

`?`를 사용할 수 있으므로, `Vec`에 대한 `fmt::Display`를 구현하는 것은 간단하다:

```rust
use std::fmt; // `fmt` 모듈 등록

// `Vec`를 포함하는 `List`라는 구조체를 정의한다.
struct List(Vec<i32>);

impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // 튜플 인덱싱을 사용하여 값을 추출하고,
        // `vec`에 대한 참조를 생성한다.
        let vec = &self.0;

        write!(f, "[")?;

        // `vec`에서 `v`를 반복함과 동시에
        // `count`에서 반복 횟수 또한 열거한다.
        for (count, v) in vec.iter().enumerate() {
            // 첫 번째 요소를 제외한 모든 요소에 쉼표를 추가한다.
            // 오류 시 반환하기 위해선 `?` 연산자를 사용하면 된다.
            if count != 0 { write!(f, ", ")?; }
            write!(f, "{}", v)?;
        }

        // 열린 대괄호를 닫고 `fmt::Result` 값을 반환하자.
        write!(f, "]")
    }
}

fn main() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}
```

### **활동**

벡터에 있는 각 요소의 인덱스도 출력되도록 위 코드를 변경해보자. 새 출력 결과는 다음과 같아야 한다.

```text
[0: 1, 1: 2, 2: 3]
```

---

## **서식 지정(Formatting)**

우리는 위에서 포맷 문자열을 통해 서식을 지정한다는 것을 보았다.

- format!("{}", foo) -> "3735928559"
- format!("0x{:X}", foo) -> "[0xDEADBEEF](https://en.wikipedia.org/wiki/Deadbeef#Magic_debug_values)"
- format!("0o{:o}", foo) -> "0o33653337357"

동일한 변수 (foo)는 사용되는 인수 타입에 따라 포맷이 달라질 수 있다: "X" vs "o" vs "지정되지 않음"

이 서식 지정 기능은 특성(Traits)들을 통해 구현되며, 각 인수 타입마다 하나의 특성이 있다. 가장 일반적인 서식 지정 특성은 인수 타입이 지정되지 않은 경우를 처리하는 `Display`이다: 예를 들면 `{}`이다.

```rust
use std::fmt::{self, Formatter, Display};

struct City {
    name: &'static str,
    // Latitude (위도)
    lat: f32,
    // Longitude (경도)
    lon: f32,
}

impl Display for City {
    // `f`는 버퍼이며, 이 메서드는 형식이 지정된 문자열을 이 버퍼에 기록해야 한다.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let lat_c = if self.lat >= 0.0 { 'N' } else { 'S' };
        let lon_c = if self.lon >= 0.0 { 'E' } else { 'W' };

        // `write!`는 `format!`과 비슷하지만, 
        // 형식이 지정된 String을 버퍼(첫 번째 인수)에 기록한다.
        write!(f, "{}: {:.3}°{} {:.3}°{}",
               self.name, self.lat.abs(), lat_c, self.lon.abs(), lon_c)
    }
}

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

fn main() {
    for city in [
        City { name: "Dublin", lat: 53.347778, lon: -6.259722 },
        City { name: "Oslo", lat: 59.95, lon: 10.75 },
        City { name: "Vancouver", lat: 49.25, lon: -123.1 },
    ] {
        println!("{}", city);
    }
    for color in [
        Color { red: 128, green: 255, blue: 90 },
        Color { red: 0, green: 3, blue: 254 },
        Color { red: 0, green: 0, blue: 0 },
    ] {
        // fmt::Display에 대한 구현을 추가한 후에는 `{}`를 사용하도록 전환한다.
        println!("{:?}", color);
    }
}
```

[서식 지정 특성의 전체 목록](https://doc.rust-lang.org/std/fmt/#formatting-traits)과 해당 인수 타입은 `std::fmt` 문서에서 확인할 수 있다.

### **활동**

위의 Color 구조체에 대한 `fmt::Display` 특성 구현을 추가하여 출력이 다음과 같이 표시되도록 해보자:

```text
RGB (128, 255, 90) 0x80FF5A
RGB (0, 3, 254) 0x0003FE
RGB (0, 0, 0) 0x000000
```

만약 조금 어렵다면, 아래 두 개의 힌트를 참고해보자:

- [각 색상을 두 번 이상 나열해야 할 수도 있다.](https://doc.rust-lang.org/std/fmt/#named-parameters)
- `:0>2`를 사용하여 [width(너비) 2까지 0으로 채울 수 있다.](https://doc.rust-lang.org/std/fmt/#width)

---

참고 링크: [Hello World](https://doc.rust-lang.org/stable/rust-by-example/hello.html)
