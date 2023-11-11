use std::collections::HashMap;

fn main() {
    // 선언 및 초기화: Rust에서는 타입을 명시적으로 선언해야 한다.
    // 여기서는 i32 타입의 키와 값을 갖는 HashMap을 생성한다.
    let mut hash_map: HashMap<i32, i32> = HashMap::new();

    // 초기값을 가진 HashMap을 생성하는 다른 방법:
    let mut hash_map: HashMap<i32, i32> = [(1, 2), (5, 3), (7, 2)].iter().cloned().collect();

    // 키의 존재 여부를 확인하려면 contains_key 메소드를 사용한다:
    println!("{}", hash_map.contains_key(&1)); // true 출력
    println!("{}", hash_map.contains_key(&9)); // false 출력

    // 주어진 키로 값에 접근하려면, get 메소드를 사용하고 결과를 unwrap한다.
    // 값이 없는 경우 대비해 unwrap_or 메소드로 기본값을 제공한다.
    println!("{}", hash_map.get(&5).unwrap_or(&-1)); // 3 출력

    // 키에 값을 추가하거나 업데이트하려면 insert 메소드를 사용한다.
    hash_map.insert(5, 6);

    // 키가 존재하지 않을 경우, 새 키-값 쌍이 삽입된다.
    hash_map.insert(9, 15);

    // 키를 삭제하려면 remove 메소드를 사용한다.
    hash_map.remove(&9);

    // 해시 맵의 크기를 알아보려면 len 메소드를 사용한다.
    println!("{}", hash_map.len()); // 3 출력

    // 키와 값의 쌍을 순회하며 출력하려면 for 루프를 사용한다.
    for (key, value) in &hash_map {
        println!("{} {}", key, value);
    }
}
