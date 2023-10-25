---
title: "[Rust] 숫자 맞추기 게임"
categories: [Rust 연구소]
tags: [Rust]
date: 2023-02-01 01:30
img_path: /assets/img/rust/
---

---

![Title](rust_title.png)

---

## **개요**

Rust의 기본 개념들을 실제 프로그램 작성을 통해 소개하고자 한다. 우리가 다루게 될 내용은 `let`, `match`, 메소드, 관련 함수, 그리고 외부 크레이트 사용법 등이다. 그리고 이어지는 글에서, 이 개념들을 더 깊게 이해할 수 있도록 설명할 예정이다.

"숫자 맞추기 게임"이라는 전통적인 초보자용 프로그래밍 문제를 통해 위 개념들을 살펴볼 것이다. 이 게임의 규칙은 간단하다. 프로그램은 먼저 1부터 100 사이의 임의의 숫자를 생성한다. 이후 유저에게 숫자를 추측하여 입력하라는 안내를 한다. 유저가 추측한 숫자가 입력되면, 프로그램은 그 숫자가 너무 낮은지, 높은지, 아니면 정확한지를 알려준다. 유저의 추측이 정답과 일치한다면, 프로그램은 축하의 메시지를 출력하고, 종료된다.

---

## **새 프로젝트 설정**

새 프로젝트를 설정하기 위해, 아래와 같이 Cargo를 사용하여 새 프로젝트를 만들어보자.

```bash
$ cargo new guessing_game
Created binary (application) `guessing_game` package
$ cd guessing_game
```

첫 번째 명령어인 `cargo new`는 프로젝트 이름(`guessing_game`)을 첫 번째 인수로 받는다. 두 번째 명령어는 새 프로젝트의 디렉토리로 이동한다.

생성된 *Cargo.toml* 파일을 살펴보자.

파일명: *Cargo.toml*

```toml
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2021"

# 더 많은 키와 그 정의를 보려면 https://doc.rust-lang.org/cargo/reference/manifest.html을 참조

[dependencies]
```

`cargo new`는 "Hello, world!" 프로그램을 생성한다. 이번에는 *src/main.rs* 파일을 확인하자.

파일명: *src/main.rs*

```rs
fn main() {
    println!("Hello, world!");
}
```

이제 `cargo run` 명령어를 사용하여 이 "Hello, world!" 프로그램을 컴파일하고 실행해보자.

```bash
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 0.46s
     Running `target/debug/guessing_game`
Hello, world!
```

`run` 명령어는 이 게임에서와 같이 프로젝트를 빠르게 반복할 필요가 있을 때 유용하며, 각 루프를 빠르게 테스트하고 다음 단계로 넘어갈 수 있게 해준다.

*src/main.rs* 파일을 다시 열어보자. 우리는 이 파일에 코드를 작성할 것이다.

---

## **추측 프로세스**

숫자 맞추기 게임 프로그램의 첫 부분은 유저로부터 입력을 요청하고, 그 입력을 처리하며, 입력이 예상된 형태인지 확인하는 부분이다. 우선, 프로그램을 시작하기 위해, 유저에게 추측을 입력하도록 허용해야 한다. 아래 코드를 *src/main.rs*에 작성해보자.

파일명: *src/main.rs*

```rs
// 유저로부터 추측을 얻어와 출력하는 코드

use std::io;

fn main() {
    println!("숫자를 맞춰보세요!");

    println!("당신이 추측한 숫자를 입력하세요.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("해당 줄을 읽지 못했습니다.");

    println!("당신이 추측한 숫자: {guess}");
}
```

이 코드에는 많은 정보가 담겨 있다. 그러므로 각 줄마다 자세히 살펴볼 필요가 있다. 유저로부터 입력을 얻고 결과를 출력하려면 `io` 입력/출력 라이브러리를 범위에 포함시켜야 한다. `io` 라이브러리는 표준 라이브러리인 `std`에서 제공된다:

```rs
use std::io;
```

Rust는 기본적으로 모든 프로그램에 표준 라이브러리에서 정의된 일련의 요소를 포함한다. 이러한 집합을 *prelude*라고 부르며, [표준 라이브러리 문서](https://doc.rust-lang.org/std/prelude/index.html)에서 확인할 수 있다.

만약 필요한 타입이 prelude에 포함되어 있지 않은 경우, 해당 타입을 스코프에 명시적으로 포함시키기 위해 `use` 문을 사용해야 한다. `std::io` 라이브러리를 이용하면, 유저 입력을 처리하는데 필요한 다양한 기능을 제공받을 수 있게 된다.

`main` 함수는 프로그램의 시작 지점이다.

```rs
fn main() {
```

`fn` 구문은 새 함수를 선언하며, 괄호`()`는 매개변수가 없음을 나타내고, 중괄호 `{`는 함수의 본문이 시작된다는 것을 나타낸다.

`println!`은 화면에 문자열을 출력하는 매크로이다:

```rs
    println!("숫자를 맞춰보세요!");

    println!("당신이 추측한 숫자를 입력하세요.");
```

이 코드는 게임의 내용을 알리고 유저에게 입력을 요청하는 메시지를 출력한다.

---

### **변수를 사용하여 값 저장**

다음으로, 유저 입력을 저장할 변수를 만든다:

```rs
    let mut guess = String::new();
```

이 줄에서는 많은 일이 일어나고 있다. 우리는 `let` 문을 사용하여 변수를 생성한다. 아래에는 위와 다른 예시가 있다:

```rs
let apples = 5;
```

이 줄은 새 변수인 `apples`를 생성하고 값 5를 바인딩한다. Rust에서의 변수는 기본적으로 불변이다. 즉, 변수에 값을 할당하고 나면, 그 값은 변경되지 않는다. 이 개념에 대해서는 향후 더 자세히 논의할 것이다. 변수를 가변으로 만들려면 변수 이름 앞에 `mut`을 추가해야 한다:

```rs
let apples = 5;  // 불변
let mut bananas = 5;  // 가변
```

> 참고: `//` 구문은 해당 줄의 끝까지 주석 처리를 한다. Rust는 주석 내의 모든 것을 무시한다.
{: .prompt-tip }

숫자 맞추기 게임 프로그램으로 돌아가 보자. 이제 `let mut guess` 구문이 `guess`라는 이름의 가변 변수를 도입했음을 알 수 있다. 등호(`=`)는 Rust에게 변수에 무언가를 바인딩하려고 한다는 것을 알려준다. 등호의 오른쪽은 `guess`가 바인딩될 값으로, `String::new` 호출의 결과가 `guess`에 바인딩 될 것이다. 이 함수는 새로운 `String` 인스턴스를 반환한다. [String](https://doc.rust-lang.org/std/string/struct.String.html)은 표준 라이브러리에서 제공하는 문자열 타입으로, 확장 가능하고 UTF-8로 인코딩된 문자열이다.

`::new` 부분의 `::` 구문은 `new`가 `String` 타입의 연관 함수임을 나타낸다. 이 경우 연관 함수는 `String`에 대해 구현된 함수이다. 이 `new` 함수는 새로운 빈 문자열을 생성한다. 이러한 `new` 함수는 어떤 종류의 새 값을 만드는 함수에 대한 일반적인 이름이기 때문에 많은 타입에서도 쉽게 찾아볼 수 있다.

간단히 말해서, `let mut guess = String::new();`는 새로운 빈 `String`을 만들고, 이를 `guess`라는 이름의 변경 가능한 변수에 저장하는 것이다.

---

### **유저 입력 받기**

프로그램의 첫번째 줄에 `use std::io;`를 이용하여 표준 라이브러리의 입력/출력 기능을 포함시켰다. 이제 `io` 모듈에서 `stdin` 함수를 호출하면, 유저 입력을 처리할 수 있다.

```rs
    io::stdin()
        .read_line(&mut guess)
```

프로그램이 시작될 때 `use std::io;`를 통해 `io` 라이브러리를 불러온다. 만약 이 과정을 거치지 않았다면, 함수 호출을 `std::io::stdin`이라고 작성해야 했을 것이다. `stdin` 함수는 터미널의 표준 입력을 다루는 `std::io::Stdin` 인스턴스를 반환한다.

다음 줄의 `.read_line(&mut guess)`는 유저의 입력을 받기 위해 사용된다. 이 코드는 표준 입력으로부터 데이터를 읽어서 `guess`라는 변수에 저장하는 작업을 수행한다. 여기서 `&mut guess`는 `read_line` 함수에게 유저 입력을 저장할 위치를 알려주는 역할을 한다. 중요한 점은, `read_line`은 새로운 데이터를 추가하되 기존의 문자열 내용을 지우지 않는다는 것이다. 이 함수가 문자열을 변경할 수 있도록, `guess`는 가변적(`mutable`)이어야 한다.

여기서 사용된 `&` 기호는 해당 인자가 참조임을 나타낸다. 이는 데이터의 실제 복사본을 만들지 않고도, 메모리 상의 데이터에 접근할 수 있도록 해준다. 참조는 복잡할 수 있지만, Rust의 이점 중 하나는 참조의 안전하고 편리한 사용을 제공한다는 것이다. 이 프로그램을 완성하기 위해서 참조의 모든 세부 사항을 알 필요는 없다. 변수처럼 참조도 기본적으로 불변이며, 이를 변경 가능하게 만들기 위해서는 `&guess`가 아닌 `&mut guess`를 사용해야 한다는 점만 이해하고 있으면 된다. 이에 대한 자세한 내용은 추후 자세히 다룰 예정이다.

---

### **잠재적 실패 처리하기: Result 사용**

우리는 지금 아래 부분을 작업하는 중이었다. 이제 다음 메서드에 대해 논의해보자:

```rs
        .expect("해당 줄을 읽지 못했습니다.");
```

이 코드는 아래와 같이 작성할 수도 있다:

```rs
io::stdin().read_line(&mut guess).expect("해당 줄을 읽지 못했습니다.");
```

하지만, 줄 하나가 너무 길어지면 가독성이 떨어지기 때문에 분할하는 것이 좋다. 메서드를 `.method_name()` 구문으로 호출할 때는 새 줄과 공백을 사용하여 하나의 긴 줄을 여러 줄로 분할하는 것이 바람직하다. 이제 해당 줄의 기능을 살펴보자.

이전에 언급했듯이, `read_line`은 우리가 넘긴 문자열에 유저가 입력한 내용을 넣지만, 동시에 `Result` 값을 반환한다. `Result`는 `enum`(열거형)이라고도 하며, 여러 상태 중 하나를 가질 수 있는 타입이다. 여기서 각 상태를 *variant*라 한다.

`Result` 타입의 목적은 에러 처리 정보를 담는 것이다.

`Result`의 variant에는 `Ok`와 `Err` 두 가지가 있다. `Ok`는 작업이 성공적으로 수행됐음을 나타내며, 내부에는 생성된 값이 들어 있다. 반면, `Err`는 작업이 실패했음을 의미하고, 실패의 원인이나 방법에 대한 정보를 포함한다.

`Result` 타입의 값은 다른 타입처럼 미리 정의된 메서드들을 갖고 있다. 이 중 `expect`라는 메서드는 `Result` 인스턴스에 사용할 수 있다. `Result` 값이 `Err`일 경우, `expect` 메서드는 프로그램을 종료하고 해당 메서드에 전달된 메시지를 화면에 출력한다. 이런 상황은 주로 `read_line`이 운영 체제 관련 에러 때문에 `Err`를 반환할 때 발생한다. 반면, `Result` 값이 `Ok`이면, `expect` 메서드는 이 Ok 값 안에 들어있는 값을 반환하며, 이 값은 사용자가 입력한 내용의 바이트 수를 나타낸다.

만약 `expect`를 호출하지 않는다면, 프로그램이 컴파일 되긴 하지만 아래와 같은 경고 메시지를 받게 될 것이다.

```bash
$ cargo build
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
warning: unused `Result` that must be used
  --> *src/main.rs*:10:5
   |
10 |     io::stdin().read_line(&mut guess);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: this `Result` may be an `Err` variant, which should be handled
   = note: `#[warn(unused_must_use)]` on by default

warning: `guessing_game` (bin "guessing_game") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 0.59s
```

Rust는 `read_line`에서 반환된 `Result` 값을 사용하지 않았다고 경고하며, 프로그램에서 발생 가능한 에러를 처리하지 않았음을 지적한다.

경고를 제거하는 적절한 방법은 에러 처리 코드를 작성하는 것이다. 여기서는, 문제가 발생했을 때 프로그램이 종료되기를 원하므로, `expect` 함수를 사용한다. 에러 복구 방법은 나중에 더 자세히 배울 수 있다.

---

### **println!을 이용하여 값을 출력하기**

우리가 논의해야 할 한 줄이 더 남아 있다(닫는 중괄호는 제외):

```rs
    println!("당신이 추측한 숫자: {guess}");
```

이 줄은 유저가 입력한 문자열을 화면에 출력한다. `{}`는 자리 표시자의 역할을 하여, 출력할 값이 들어갈 위치를 지정한다. 변수의 값을 출력하려면, 해당 변수의 이름을 `{}` 안에 넣으면 된다. 특정 표현식의 결과를 출력하려면, 먼저 문자열 포맷에 `{}`를 추가하고, 그 다음 쉼표로 구분된 표현식 목록을 제공하면 된다. 이 방식을 사용하면, 변수와 표현식의 결과를 `println!` 함수 호출 하나로 출력할 수 있다.

```rs
let x = 5;
let y = 10;

println!("x = {x} and y + 2 = {}", y + 2);
```

이 코드는 `x = 5 and y + 2 = 12`를 출력한다.

---

### **첫 부분 테스트하기**

숫자 맞추기 게임의 첫 부분을 테스트해야 한다. `cargo run` 명령어를 사용하여 실행해보자:

```bash
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 6.44s
     Running `target/debug/guessing_game`
숫자를 맞춰보세요!
추측한 숫자를 입력하세요.
6
당신이 추측한 숫자: 6
```

이 시점에서 게임의 첫 부분은 완료되었다. 키보드로부터 입력을 받아 출력하는 부분까지 구현하였다.

---

## **비밀 숫자 생성하기**

다음 작업은 유저가 추측하게 될 비밀 숫자를 생성하는 것이다. 게임을 여러 번 플레이해도 재미있게 하기 위해서는 비밀 숫자가 매번 달라져야 한다. 게임이 너무 어렵지 않게 1부터 100 사이의 임의의 숫자를 사용할 것이다. 현재 Rust의 표준 라이브러리에는 난수 생성 기능이 없다. 하지만, Rust 팀은 이 기능을 제공하는 [rand](https://crates.io/crates/rand) 크레이트를 제공하고 있다.

---

### **크레이트를 통해 기능 확장하기**

크레이트는 Rust의 소스 코드 모음이라는 점을 명심해야 한다. 지금까지 우리가 작성한 프로젝트는 실행 가능한 파일을 만드는 바이너리 크레이트이다. 그와 대조적으로, `rand` 크레이트는 다른 프로그램에서 활용 가능한 코드를 포함하는 라이브러리 크레이트로, 자체적으로는 실행할 수 없다.

여기서 Cargo의 유용성이 드러난다. `rand` 크레이트를 사용하려면 먼저 *Cargo.toml* 파일을 편집하여 rand 크레이트를 의존성 목록에 추가해야 한다. 이 파일을 열고, Cargo가 준비한 `[dependencies]` 섹션 바로 아래에 다음 내용을 추가한다. 이 글에서 제시하는 코드 예제와 일치하도록 여기서 지정한 버전 번호를 정확히 따라야 한다:

파일명: *Cargo.toml*

```toml
[dependencies]
rand = "0.8.5"
```

*Cargo.toml* 파일 내의 각 섹션은 헤더 아래에 위치한 내용으로 구성되며, 다음 섹션이 시작될 때까지 계속된다. `[dependencies]` 섹션은 프로젝트에 필요한 외부 크레이트와 그에 대한 특정 버전을 명시한다. 여기서는 `rand` 크레이트에 대해 `"0.8.5"` 버전을 지정하는데, 이는 시맨틱 버전 규칙을 따르는 것이다. Cargo는 이 [시맨틱 버전](http://semver.org/) 을 해석할 수 있다. `"0.8.5"`라고 표현하면 실제로는 `"^0.8.5"`를 의미하는데, 이는 0.8.5보다 높고 0.9.0보다 낮은 버전을 모두 포함한다.

Cargo는 이런 버전들이 0.8.5와 호환되는 공개적인 API를 제공한다고 판단한다. 이는 현재 챕터의 코드에 적합한 최신 수정 버전을 자동으로 받을 수 있음을 의미하지만, 0.9.0 이상 버전에서 동일한 API가 보장되지는 않는다는 것을 의미한다.

이제 코드 수정 없이 프로젝트를 빌드해볼 차례다. 아래와 같이 진행하면 된다.

```bash
$ cargo build
    Updating crates.io index
  Downloaded rand v0.8.5
  Downloaded libc v0.2.127
  Downloaded getrandom v0.2.7
  Downloaded cfg-if v1.0.0
  Downloaded ppv-lite86 v0.2.16
  Downloaded rand_chacha v0.3.1
  Downloaded rand_core v0.6.3
   Compiling libc v0.2.127
   Compiling getrandom v0.2.7
   Compiling cfg-if v1.0.0
   Compiling ppv-lite86 v0.2.16
   Compiling rand_core v0.6.3
   Compiling rand_chacha v0.3.1
   Compiling rand v0.8.5
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53s
```

우리가 확인하는 버전 번호가 위에서 언급한 것과 다를 수도 있지만, SemVer 덕분에 이 버전들은 우리 코드와 호환된다. 운영 체제에 따라 출력되는 줄이 다르거나, 줄의 순서가 바뀔 수 있다.

외부 의존성을 포함할 때, Cargo는 해당 의존성이 요구하는 모든 것들의 최신 버전을 레지스트리에서 가져온다. 이 레지스트리는 Crates.io에서 데이터를 복사한 것으로, [Crates.io](https://crates.io/)는 Rust 커뮤니티가 자신의 오픈 소스 Rust 프로젝트를 공유하고 다른 사람들이 사용할 수 있도록 하는 플랫폼이다.

레지스트리가 업데이트된 후, Cargo는 `[dependencies]` 섹션을 확인하고 아직 다운로드되지 않은 크레이트를 가져온다. 여기서는 `rand`만 의존성 목록에 추가했지만, Cargo는 `rand`가 동작하는 데 필요한 다른 모든 크레이트도 함께 다운로드한다. 크레이트가 다운로드되면, Rust는 이를 컴파일하고, 우리 프로젝트도 이러한 의존성을 사용할 수 있도록 컴파일한다.

변경 사항이 없으면 `cargo build`를 다시 실행해도, `Finished`문구 외에는 별다른 출력이 없을 것이다. Cargo는 이미 의존성들을 다운로드하고 컴파일했으며, *Cargo.toml* 파일이 변경되지 않았음을 인식한다. 코드에도 변동이 없으므로, 다시 컴파일할 필요가 없다. 따라서 Cargo는 명령 실행을 여기서 종료한다.

*src/main.rs* 파일을 그 상태로 바로 빌드하면, 다음과 같은 두 줄의 출력을 확인할 수 있다:

```bash
$ cargo build
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53 secs
```

이 출력은 Cargo가 *src/main.rs* 파일에서 변경된 부분만을 대상으로 빌드를 업데이트한다는 것을 나타낸다. 의존성에 변화가 없으므로, Cargo는 이전에 다운로드하고 컴파일한 내용을 재사용할 수 있다고 판단한다.

---

### **Cargo.lock 파일로 안정적인 빌드 환경 확보**

Cargo에는 누구든 코드를 빌드할 때마다 동일한 결과물을 만들 수 있는 기능이 내장되어 있다. 예를 들어, `rand` crate의 새로운 0.8.6 버전이 출시되었다고 가정해보자. 이 버전에는 중요한 버그 수정이 포함되어 있지만, 당신의 코드를 망가뜨릴 수 있는 에러도 함께 있을 수 있다. 이런 문제를 방지하기 위해 Rust는 `cargo build` 명령어를 처음 실행할 때 *Cargo.lock* 파일을 생성하며, 이는 *guessing_game* 디렉토리에 저장된다.

프로젝트를 처음 빌드할 때, Cargo는 필요한 의존성 버전을 확인하고 이 정보를 *Cargo.lock* 파일에 저장한다. 그 후 프로젝트를 다시 빌드할 때, Cargo는 이 *Cargo.lock* 파일이 있다는 것을 인지하고, 여기에 명시된 버전을 사용하여 불필요한 버전 확인 작업을 반복하지 않는다. 이렇게 함으로써, 빌드 과정이 항상 동일하게 재현될 수 있도록 보장한다. 다시 말해, *Cargo.lock* 파일 덕분에 유저가 직접 업그레이드하기로 결정할 때까지 프로젝트는 0.8.5 버전을 유지한다. *Cargo.lock* 파일은 빌드의 안정성을 보장하기 위해 중요하기 때문에, 프로젝트와 함께 버전 관리 시스템에 등록해야 한다(되도록 .ignore하지 말자).

---

### **크레이트 업데이트로 새 버전 얻기**

Cargo는 크레이트의 업데이트를 수행하기 위해 `update` 명령어를 제공한다. 이 명령은 *Cargo.lock* 파일을 무시하고 *Cargo.toml*의 규격에 맞는 최신 버전을 모두 파악한다. 그리고나서 Cargo는 그 버전들을 *Cargo.lock* 파일에 작성한다. 만약 이 과정 없이 진행한다면, Cargo는 기본적으로 0.8.5 이상 0.9.0 미만의 버전만을 찾게 된다. 만약 `rand` crate에서 0.8.6과 0.9.0이라는 두 가지 새 버전을 출시했다면, `cargo update`를 실행했을 때 다음과 같은 결과를 볼 수 있다:

```bash
$ cargo update
    Updating crates.io index
    Updating rand v0.8.5 -> v0.8.6
```

이때 Cargo는 0.9.0 버전을 무시한다. 이 때 개발자는 *Cargo.lock* 파일에서 `rand` crate의 버전이 0.8.6으로 변경된 것을 확인할 수 있다. `rand`의 0.9.0 버전이나 0.9.x 시리즈 중 어떤 버전을 사용하고자 한다면, *Cargo.toml* 파일을 다음과 같이 바꿔야 한다:

```toml
[dependencies]
rand = "0.9.0"
```

이후 `cargo build`를 실행하면, Cargo는 사용 가능한 크레이트들의 목록을 업데이트하고, 설정한 새 버전에 맞게 rand에 대한 요구 조건을 다시 검토한다.

[Cargo](http://doc.crates.io/)와 [Crate](http://doc.crates.io/crates-io.html)에 대해 많은 것을 논의할 수 있지만, 현재로서 필요한 정보는 이 정도이다. Cargo는 라이브러리를 재사용하기 쉽게 만들어주기 때문에, Rust 개발자들은 여러 개의 패키지를 조합해 작은 프로젝트를 만들 수 있다.

---

### **임의의 숫자 생성하기**

rand 라이브러리를 사용하여 추측할 숫자를 생성해보자. 다음 단계는 *src/main.rs*를 업데이트하는 것이다.

파일명: *src/main.rs*

```rs
// 임의의 숫자를 생성하기 위한 코드 추가

use std::io;
use rand::Rng;

fn main() {
    println!("숫자를 맞춰보세요!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("비밀 숫자는: {secret_number}");

    println!("당신이 추측한 숫자를 입력하세요.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("해당 줄을 읽지 못했습니다.");

    println!("당신이 추측한 숫자: {guess}");
}
```

우선 `use rand::Rng;` 구문을 추가해야 한다. `Rng` 트레잇은 난수 생성기를 구현할 수 있는 여러 메서드를 정의하고 있다. 이 트레잇은 해당 메서드들을 우리 코드에서 사용하기 위해 스코프 내에 반드시 포함되어 있어야 한다. 트레잇이 무엇인지, 어떻게 작동하는지에 대한 더 자세한 설명은 이후에 다룰 예정이다.

다음으로, 코드의 중간 부분에 다음 두 줄을 추가한다:

```rs
let mut rng = rand::thread_rng();
let secret_number = rng.gen_range(1..=100);
```

첫 번째 줄에서는, 우리가 사용할 난수 생성기를 초기화하는 `rand::thread_rng` 함수를 호출한다. 이 함수는 현재 실행 중인 스레드에 지역적인 난수 생성기를 제공하며, 이는 운영 체제로부터 시드를 받아 초기화된다.

두 번째 줄에서는, 이 난수 생성기의 `gen_range` 메서드를 호출하여 임의의 숫자를 생성한다. 이 메서드는 `use rand::Rng;` 구문을 통해 현재 스코프로 가져온 `Rng` 트레잇에 의해 제공된다. `gen_range`는 입력으로 받은 범위 내에서 숫자를 생성하며, 여기서는 `1..=100`이라는 범위를 지정하고 있다. 이 표현식은 "1 이상 100 이하"를 의미하며, 이 범위 내의 임의의 숫자를 반환한다.

> **참고**: 어떤 트레잇을 써야 할지, 크레이트에서 어떤 메서드나 함수를 호출해야 할지 바로 알기 어렵다. 이 때 사용하기 좋은 기능이 하나 있다.
>
> 모든 크레이트는 사용 방법을 알려주는 문서를 갖추고 있다. Cargo에는 흥미로운 기능이 있는데, 바로 `cargo doc --open` 명령이다.
>
> `cargo doc --open` 명령을 사용하면, 컴퓨터에 저장된 모든 의존성 문서를 생성하고 웹 브라우저로 바로 열 수 있다. 예를 들어, `rand` 크레이트의 추가 기능에 대해 궁금한 점이 있다면, 이 명령을 실행 후 생성된 문서에서 `rand`를 선택하여 필요한 정보를 찾아볼 수 있다.
{: .prompt-tip }

다음으로, 프로그램 코드 중 변경해야 할 부분이 있다. 개발 단계에서 비밀 숫자를 화면에 출력하는 것은 테스트하기에 유용하지만, 최종 버전에서는 이를 제거해야 한다. 프로그램이 시작되면서 바로 정답을 알려주는 것은 게임의 본질에 어긋나기 때문이다.

이제 여러 번 프로그램을 실행하여 테스트해보자.

```bash
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 2.53s
     Running `target/debug/guessing_game`
숫자를 맞춰보세요!
비밀 숫자는: 7
당신이 추측한 숫자를 입력하세요.
4
당신이 추측한 숫자: 4

$ cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/guessing_game`
숫자를 맞춰보세요!
비밀 숫자는: 83
당신이 추측한 숫자를 입력하세요.
5
당신이 추측한 숫자: 5
```

서로 다른 임의의 숫자가 생성되어야 하며, 이 숫자들은 모두 1에서 100 사이에 속해야 한다.

---

## **비밀 숫자와 추측한 숫자 비교하기**

유저 입력과 임의의 숫자가 준비되었으므로, 이제 이들를 비교할 차례이다.

파일명: *src/main.rs*

> 아래 코드는 컴파일되지 않는다.
{: .prompt-danger }

```rs
// 두 숫자 비교의 가능한 반환 값을 처리하는 방법

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // --생략--

    println!("당신이 추측한 숫자: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("숫자가 너무 작습니다!!"),
        Ordering::Greater => println!("숫자가 너무 큽니다!!"),
        Ordering::Equal => println!("숫자를 맞췄습니다!"),
    }
}
```

우선, 표준 라이브러리에서 `std::cmp::Ordering`타입을 사용하기 위해 새로운 `use` 구문을 추가한다. `Ordering` 타입은 열거형으로, `Less`, `Greater`, `Equal` 세 가지 variants를 갖는다. 이것은 두 값을 비교했을 때 가능한 세 가지 결과를 나타낸다.

이어서, `Ordering` 타입을 활용하는 새로운 코드를 아래에 추가한다. `cmp` 메서드는 두 값을 비교하고, 비교 가능한 모든 항목에 대해 호출할 수 있다. 이것은 비교하려는 대상에 대한 참조를 매개변수로 받는다: 여기서는 `guess`와 `secret_number`를 비교한다. 그리고나서, `use` 구문을 통해 가져온 `Ordering` 열거형의 적절한 variants을 반환한다. `match` 표현식을 사용하여 `cmp` 호출에서 반환된 `Ordering`의 variants에 따라 다음에 수행할 작업을 결정한다.

`match` 표현식은 *여러 개의 분기(arms)*로 구성된다. 각 분기는 *패턴*과, `match`로 전달된 값이 해당 분기의 패턴과 일치하는 경우 실행할 코드를 포함한다. Rust는 `match`로 전달된 값을 받고 각 분기의 패턴을 차례로 검사한다. 패턴과 `match` 구조는 Rust의 강력한 기능으로, 코드가 마주칠 수 있는 다양한 상황을 표현할 수 있게 해주고, 모든 상황을 처리할 수 있도록 보장한다.

여기서 사용된 `match` 표현식을 살펴보자. 유저가 50이라고 추측했고, 임의로 생성된 비밀 숫자가 38이라고 가정해보자.

50과 38을 비교할 때, `cmp` 메서드는 50이 38보다 크기 때문에 결과로 `Ordering::Greater`를 반환한다. 이후 `match` 표현식은 `Ordering::Greater` 값을 받아 분기의 패턴 검사를 시작한다. 첫 번째 분기는 `Ordering::Less` 패턴을 갖고 있지만, `Ordering::Greater`값은 이 패턴과 일치하지 않으므로, 이 분기는 건너뛰고 다음 분기로 넘어간다. 이어서 나오는 분기의 패턴은 `Ordering::Greater`로, 이는 받아들인 값과 일치한다. 따라서, 해당 분기의 코드가 실행되며, 화면에 **숫자가 너무 큽니다!!**를 출력한다. `match` 표현식은 첫 번째로 일치하는 분기를 찾은 이후 종료되기 때문에, 이 시나리오에서는 나머지 분기를 더 이상 확인하지 않는다.

그러나 이 코드는 컴파일되지 않는다. 직접 시도해 보자:

```bash
$ cargo build
   Compiling libc v0.2.86
   Compiling getrandom v0.2.2
   Compiling cfg-if v1.0.0
   Compiling ppv-lite86 v0.2.10
   Compiling rand_core v0.6.2
   Compiling rand_chacha v0.3.0
   Compiling rand v0.8.5
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
error[E0308]: mismatched types
  --> *src/main.rs*:22:21
   |
22 |     match guess.cmp(&secret_number) {
   |                 --- ^^^^^^^^^^^^^^ expected struct `String`, found integer
   |                 |
   |                 arguments to this function are incorrect
   |
   = note: expected reference `&String`
              found reference `&{integer}`
note: associated function defined here
  --> /rustc/d5a82bbd26e1ad8b7401f6a718a9c57c96905483/library/core/src/cmp.rs:783:8

For more information about this error, try `rustc --explain E0308`.
error: could not compile `guessing_game` due to previous error
```

에러 발생 원인은 타입 불일치 때문이다. Rust는 엄격한 정적 타입 시스템을 가지고 있으며, 타입 추론 기능도 함께 제공한다.

예를 들어, `let mut guess = String::new();`라고 코드를 작성하면 Rust는 `guess`가 `String` 타입임을 자동으로 판단한다. 이 경우 개발자가 별도로 타입을 명시하지 않아도 된다. 하지만, `secret_number` 같은 숫자는 `guess`와 다른 타입에 속하며, Rust에는 `i32` (32비트 정수), `u32` (부호 없는 32비트 정수), `i64` (64비트 정수)와 같은 다양한 숫자 타입을 제공한다. 특별한 지정이 없는 경우, Rust는 `i32`를 기본으로 한다. 이는 `secret_number`의 타입이기도 하다.

개발자가 다른 타입 정보를 제공하면, Rust는 이 정보를 바탕으로 타입을 추론한다. 이러한 에러가 발생하는 근본적인 이유는 Rust가 문자열과 숫자 타입을 서로 비교할 수 없기 때문이다. 이 문제를 정확히 파악하고 적절한 해결책을 찾을 수 있어야 한다.

결국, 우리는 프로그램이 입력으로 받은 `String`을 실제 숫자 타입으로 변환하여 `secret_number`와 수치적으로 비교할 수 있도록 해야 한다. 이를 위해 `main` 함수 본문에 다음 코드를 추가해야 한다.

파일명: *src/main.rs*

```rs
// --생략--

let mut guess = String::new();

io::stdin()
    .read_line(&mut guess)
    .expect("해당 줄을 읽지 못했습니다.");

let guess: u32 = guess.trim().parse().expect("숫자를 입력해주세요!");

println!("당신이 추측한 숫자: {guess}");

match guess.cmp(&secret_number) {
    Ordering::Less => println!("숫자가 너무 작습니다!!"),
    Ordering::Greater => println!("숫자가 너무 큽니다!!"),
    Ordering::Equal => println!("숫자를 맞췄습니다!"),
}
```

아래 줄이 추가되었다:

```rs
let guess: u32 = guess.trim().parse().expect("숫자를 입력해주세요!");
```

우리는 `guess`라는 새 변수를 선언한다. 그러나 이미 프로그램에는 `guess`라는 이름의 변수가 존재한다. 이 상황에서 Rust는 기존의 `guess` 값을 새 값으로 숨기는 기술, 즉 **쉐도잉**을 허용한다. 이는 `guess`라는 같은 변수명을 재사용하게 해주며, 예를 들어 `guess_str`과 `guess` 같은 두 개의 고유한 변수명을 만들 필요를 없애준다. 이러한 기능은 추후에 더 자세히 다룰 예정이지만, 현재로써는 다른 타입으로 값이 변환되길 원할 때 자주 사용되는 기능임을 알아두면 된다.

이 새로운 변수 `guess`는 `guess.trim().parse()`표현식과 관련이 있다. 여기서 `guess`는 문자열 형태로 입력값을 담고 있는 기존 `guess` 변수를 참조한다. String 인스턴스에 있는 `trim` 메서드는 문자열의 앞뒤 공백을 제거한다. 이 작업은 `guess`가 공백 없이 숫자만을 포함하도록 보장하므로, `u32`와의 비교를 가능하게 만들어 준다. 유저가 엔터 키를 눌러 `read_line`을 완료하고 추측 값을 입력하면, 문자열에는 줄바꿈 문자가 추가된다. 예를 들어, 유저가 5를 입력하고 엔터 키를 누르면, `guess`는 `5\n`의 형태를 띄게 된다. 여기서 `\n`은 줄바꿈 문자를 의미한다. (Windows에서 엔터 키를 누르는 것은 캐리지 리턴과 줄바꿈, 즉 `\r\n`을 생성한다.) `trim` 메서드는 `\n`이나 `\r\n`을 제거하여 순수한 숫자 값인 5만을 남긴다.

문자열의 `parse` 메서드는 문자열을 다른 타입으로 변환하는 기능을 제공한다. 이 메서드를 이용하여 문자열을 숫자로 바꿀 수 있다. 여기서 `let guess: u32` 구문을 사용하여 Rust에게 명확한 숫자 타입을 알려줘야 한다. 여기서 `guess` 다음의 콜론(`:`)은 이 변수의 타입을 `타입 어노테이션`을 통해 지정한다는 것을 Rust에게 알려준다. Rust에는 다양한 내장 숫자 타입이 있으며, 이 경우에는 `u32`는 부호 없는 32비트 정수 타입을 사용한다. 이 타입은 비교적 작은 양의 정수를 다루기에 적합하다. 다른 숫자 타입들에 대해서는 추후에 자세히 알아볼 예정이다.

또한, 이 예제 프로그램에서 `u32`타입 어노테이션을 사용하고 `secret_number`와 비교함으로써 Rust는 `secret_number` 역시 `u32`타입이라고 판단한다. 그 결과, 이제 두 값은 동일한 타입으로 비교될 수 있다.

`parse` 메서드는 숫자로 변환할 수 있는 문자열에서만 정상 작동하며, 그렇지 않은 경우 에러를 발생시킬 수 있다. 예를 들어, 문자열에 `A👍%`와 같은 문자를 포함하고 있다면, 이를 숫자로 바꾸는 것은 불가능하다. 이러한 상황에 대비하여, `parse` 메서드는 실패할 가능성이 있는 경우, `read_line` 메서드처럼 `Result` 타입을 반환한다. 이 `Result`는 `expect` 메서드를 사용하여 처리할 수 있다. `parse`가 문자열을 성공적으로 숫자로 변환하면, `Result`의 `Ok`값이 반환되고, `expect`메서드는 이 `Ok` 값을 통해 우리가 필요로 하는 숫자를 반환한다.

이제 프로그램을 실행해보자.

```bash
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 0.43s
     Running `target/debug/guessing_game`
숫자를 맞춰보세요!
비밀 숫자는: 58
당신이 추측한 숫자를 입력하세요.
  76
당신이 추측한 숫자: 76
숫자가 너무 큽니다!!
```

입력에 공백이 추가되었음에도 불구하고, 프로그램은 유저가 76이라고 추측한 것을 정확하게 파악했다. 프로그램을 여러 번 실행하여 다양한 입력에 따른 다른 행동을 확인해보자: 숫자를 정확하게 추측하거나, 너무 높은 숫자를 추측하거나, 너무 낮은 숫자를 추측해보면 된다.

현재 게임의 대부분은 잘 작동하지만, 유저는 단 한 번만 추측할 수 있다. 이를 바꾸기 위해 루프를 추가해보자.

---

## **여러 번 추측할 수 있도록 루프 추가하기**

`loop` 키워드는 무한 루프를 생성한다. 유저들에게 숫자를 맞추는 데 더 많은 기회를 주기 위해서 루프를 추가하자:

파일명: *src/main.rs*

```rs
// --생략--

println!("비밀 숫자는: {}", secret_number);

loop {
    println!("당신이 추측한 숫자를 입력하세요.");

    // --생략--

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("숫자가 너무 작습니다!!"),
        Ordering::Greater => println!("숫자가 너무 큽니다!!"),
        Ordering::Equal => println!("숫자를 맞췄습니다!"),
    }
}
```

이렇게, 추측을 입력받는 부분부터 모든 과정을 loop 내부로 옮겼다. loop 내부의 코드는 기존의 들여쓰기에서 추가로 네 칸 더 들여써야 한다. 위와 같이 수정한 후 프로그램을 다시 실행해보면, 프로그램이 계속해서 새로운 추측을 요청하는 것을 볼 수 있다. 하지만, 이렇게 되면 새로운 문제가 발생한다. 유저가 프로그램을 어떻게 종료해야 할지 분명하지 않다.

유저는 언제든지 키보드 단축키인 `ctrl-c`를 이용하여 프로그램 실행을 중단할 수 있다. 그러나 “비밀 숫자와 추측 비교하기”에서 언급한 `parse`에 대한 논의에 따르면, 이 탈출할 수 없는 문제로부터 벗어나는 또 다른 방법이 존재한다. 만약 유저가 숫자가 아닌 값을 입력하면, 프로그램은 충돌할 것이다. 이 점을 이용하면 다음과 같이 유저가 프로그램을 종료할 수 있는 기회를 제공할 수 있다:

```bash
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 1.50s
     Running `target/debug/guessing_game`
숫자를 맞춰보세요!
비밀 숫자는: 59
당신이 추측한 숫자를 입력하세요.
45
당신이 추측한 숫자: 45
숫자가 너무 작습니다!!
당신이 추측한 숫자를 입력하세요.
60
당신이 추측한 숫자:60
숫자가 너무 큽니다!!
당신이 추측한 숫자를 입력하세요.
59
당신이 추측한 숫자:59
숫자를 맞췄습니다!
당신이 추측한 숫자를 입력하세요.
quit
thread `main` panicked at `Please type a number!: ParseIntError { kind: InvalidDigit }`, *src/main.rs*:28:47
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

`quit`이라고 입력하면 게임이 종료된다. 하지만 그렇게 하지 않고, 숫자가 아닌 다른 어떤 입력값을 입력해도 게임이 종료된다. 이 상황은 최적의 상황이 아니다; 정답을 맞췄을 때만 게임이 종료되어야 한다.

---

### **정확한 추측 후 게임 종료하기**

유저가 정답을 맞추면 게임이 종료될 수 있도록 `break` 문을 추가해보자.

파일명: *src/main.rs*

```rs
        // --생략--

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("숫자가 너무 작습니다!!"),
            Ordering::Greater => println!("숫자가 너무 큽니다!!"),
            Ordering::Equal => {
                println!("숫자를 맞췄습니다!");
                break;
            }
        }
    }
}
```

**"숫자를 맞췄습니다!"** 메시지 다음에 `break` 문을 추가하면, 유저가 비밀 숫자를 맞혔을 때 프로그램이 루프를 빠져나와 종료된다. 이 루프가 `main` 함수의 마지막 부분이므로, 루프를 종료하면 프로그램도 종료된다.

---

### **잘못된 입력 처리하기**

게임을 더 완벽하게 만들기 위해, 유저가 숫자가 아닌 값을 입력했을 때 프로그램이 충돌하지 않고, 숫자가 아닌 입력을 무시한 채 유저가 추측을 계속할 수 있도록 변경해야 한다. 아래와 같이, `guess`를 `String`에서 `u32`로 변환하는 코드를 수정해보자.

파일명: *src/main.rs*

```rs
// 숫자가 아닌 추측을 무시하고 프로그램을 충돌시키지 않고 다시 추측하도록 요청하기

        // --생략--

        io::stdin()
            .read_line(&mut guess)
            .expect("해당 줄을 읽지 못했습니다.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("당신이 추측한 숫자: {guess}");

        // --생략--
```

에러 발생 시 프로그램을 종료하는 대신 에러를 처리할 수 있도록 `expect` 호출 부분을 `match` 표현식으로 바꾸었다. `parse` 함수는 `Result` 타입을 반환하고, `Result`는 `Ok`와 `Err` 이라는 두 가지 상태를 가진 열거형이다. 이전에 `cmp` 메서드의 결과값을 다루었던 것처럼 여기서도 `match` 표현식을 사용한다.

`parse`가 문자열을 성공적으로 숫자로 변환하면, 그 결과로 `Ok` 값이 반환되며 이 값에는 변환된 숫자가 포함되어 있다. 이 `Ok` 값은 `match` 구문의 첫 번째 분기 조건과 일치하여, `parse`가 반환한 숫자를 그대로 전달한다. 이 숫자는 새로 생성하는 `guess` 변수에 저장된다.

만약 `parse`가 문자열을 숫자로 변환하지 못하는 경우, 에러 정보를 담은 `Err` 값을 반환한다. 이 Err 값은 첫 번째 분기의 `Ok(num)` 패턴과는 일치(`match`)하지 않지만 두 번째 분기의 `Err(_)` 패턴과 일치한다. 여기서 밑줄(`_`)은 모든 종류의 `Err` 값을 대표하는 와일드카드로, 이는 에러의 구체적인 내용에 상관없이 모든 `Err` 값에 대응한다는 의미이다. 따라서 프로그램은 두 번째 분기의 코드를 실행하게 되며, 프로그램에게 `loop`의 다음 회차로 넘어가서 새로운 추측을 유도하는 `continue` 명령을 내린다. 이렇게 함으로써 프로그램은 `parse` 함수에서 발생할 수 있는 모든 에러를 효과적으로 무시할 수 있게 된다.

이제 프로그램의 모든 부분이 예상대로 작동할 것이다. 실행해보자:

```bash
$ cargo run
   Compiling guessing_game v0.1.0 (file:///projects/guessing_game)
    Finished dev [unoptimized + debuginfo] target(s) in 4.45s
     Running `target/debug/guessing_game`
숫자를 맞춰보세요!
비밀 숫자는: 61
당신이 추측한 숫자를 입력하세요.
10
당신이 추측한 숫자: 10
숫자가 너무 작습니다!!
당신이 추측한 숫자를 입력하세요.
99
당신이 추측한 숫자: 99
숫자가 너무 큽니다!!
당신이 추측한 숫자를 입력하세요.
foo
당신이 추측한 숫자를 입력하세요.
61
당신이 추측한 숫자: 61
숫자를 맞췄습니다!
```

마지막으로 약간의 수정을 거치면 숫자 맞추기 게임이 완성된다. 현재 프로그램은 여전히 비밀 숫자를 출력 중이다. 테스트 단계에서는 유용했지만, 실제 게임에서는 그러면 안 된다. 비밀 숫자를 출력하는 `println!` 문을 제거하자. 아래는 이 변경사항을 포함한 최종 코드이다.

파일명: *src/main.rs*

```rs
// 숫자 맞추기 게임의 완성 코드

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("숫자를 맞춰보세요!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("당신이 추측한 숫자를 입력하세요.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("해당 줄을 읽지 못했습니다.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("당신이 추측한 숫자: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("숫자가 너무 작습니다!!"),
            Ordering::Greater => println!("숫자가 너무 큽니다!!"),
            Ordering::Equal => {
                println!("숫자를 맞췄습니다!");
                break;
            }
        }
    }
}
```

이제 숫자 맞추기 게임을 성공적으로 구축했다.

---

## **마무리**

이 프로젝트를 통해 `let`, `match`, 함수 등 Rust의 다양한 새로운 개념을 직접 작성해보았다. 특히 외부 크레이트의 사용 방법을 배울 수 있었다. 곧 우리는 이 개념들에 대해 더 깊이 파고들 것이다.

앞으로 우리는 대부분의 프로그래밍 언어에서 공통적으로 볼 수 있는 개념인 변수, 데이터 타입, 함수 등을 살펴보고, Rust에서 이러한 개념들이 어떻게 적용되는지에 대해 배울 것이다. 그리고 Rust의 독특한 특성 중 하나인 **소유권** 개념을 탐색할 것이다. 이는 Rust를 다른 프로그래밍 언어들과 구별해주는 중요한 요소이다.

그 후에는, 구조체와 메서드에 대한 문법을 다루는 방법을 배우고, 열거형이 어떻게 작동하는지에 대한 설명을 진행할 것이다. 이러한 내용들을 통해 프로그래밍에 대한 더 깊은 이해를 할 수 있을 것이다.

---

> 출처: [rust-lang book](https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html#programming-a-guessing-game)
{: .prompt-info }
