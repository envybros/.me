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