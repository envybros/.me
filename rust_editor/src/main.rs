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
            write!(f, "{}: {}", count, v)?;
        }

        // 열린 대괄호를 닫고 `fmt::Result` 값을 반환하자.
        write!(f, "]")
    }
}

fn main() {
    let v = List(vec![1, 2, 3]);
    println!("{}", v);
}