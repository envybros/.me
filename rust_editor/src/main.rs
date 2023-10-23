fn is_subsequence(s: String, t: String) -> bool {
    let mut i = 0;
    let mut j = 0;
    let s_chars: Vec<char> = s.chars().collect();
    let t_chars: Vec<char> = t.chars().collect();

    while i < s_chars.len() && j < t_chars.len() {
        if s_chars[i] == t_chars[j] {
            i += 1;
        }
        j += 1;
    }

    i == s_chars.len()
}

fn main() {
    // 테스트 케이스
    let test_cases = vec![
        ("abc".to_string(), "ahbgdc".to_string(), true),
        ("axc".to_string(), "ahbgdc".to_string(), false),
    ];

    // 모든 테스트 케이스를 실행하고 결과를 출력
    for (s, t, expected) in test_cases {
        assert_eq!(is_subsequence(s, t), expected);
    }

    println!("All tests passed.");
}