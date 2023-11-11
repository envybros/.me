---
title: "[DSA] 요소의 존재 여부 확인"
categories: [Algorithm 연구소]
tags: [Algorithm, DSA, Hashing]
date: 2022-08-12 01:10
math: true
img_path: /assets/img/algorithm/
---

---

![Title](algorithm_title.png)

---

## **개요**

해시 테이블이나 세트를 활용하는 가장 일반적인 용도 중 하나는, 어떤 요소가 존재하는지를 $O(1)$의 시간 복잡도로 확인하는 것이다. 배열을 사용할 경우 이 작업을 수행하는 데 $O(n)$이 소요되므로, 해시 맵이나 세트를 활용하면 알고리즘의 시간 복잡도를 크게 줄일 수 있다. 보통 이는 시간 복잡도를 $O(n^2)$에서 $O(n)$으로 개선하는 데 도움이 된다. 이와 관련된 몇 가지 예제 문제들을 살펴보도록 하자.

---

## **예제 1: 두 수의 합**

> [문제 링크](https://leetcode.com/problems/two-sum/)
>
> 정수로 이루어진 배열 `nums`와 정수 `target`이 주어졌을 때, 그 합이 `target`이 되는 두 숫자의 인덱스를 반환해보자. 여기서 동일한 인덱스를 두 번 사용하는 것은 허용되지 않는다.
{: .prompt-tip }

이 문제의 간단한 해결 방법은 모든 인덱스 쌍에 대해 합이 `target`과 같은지 확인하는 것이다. 이러한 방식은 $O(n^2)$의 시간 복잡도를 갖는다. 첫 번째 for 문은 특정 숫자 `num`을 선택하고, 두 번째 for 문은 배열에서 `target - num`을 찾는다. 배열을 사용할 경우 `target - num`을 찾는 데 $O(n)$이 소요되지만, 해시 맵을 사용하면 $O(1)$로 줄어든다.

배열을 순회하면서 각 값의 인덱스를 해시 맵에 매핑할 수 있다. 각 인덱스 `i`에서, `num = nums[i]`일 때, `target - num`을 해시 맵에서 찾을 수 있다. 키-값 쌍을 추가하고 `target - num`을 찾는 과정은 모두 $O(1)$이므로, 전체 시간 복잡도는 $O(n)$으로 개선된다.

### 예제 1 상세 설명

우리가 찾는 것은 `target`에 해당하는 두 숫자의 합이다. 입력값을 순회하며 각 요소 `num`이 다른 숫자와 `target`을 형성할 수 있는지 확인한다.

`target - num`에 해당하는 다른 요소가 존재한다면, 그들의 합 `num + target - num = target`이 우리가 찾고자 하는 바이다.

따라서 입력을 순회하며 요소들을 해시 맵에 저장한다. 그 후 각 `num`에 대해 `target - num`을 $O(1)$에 확인할 수 있다. 이 문제는 숫자 자체가 아닌 그 인덱스를 반환하도록 요구하기 때문에, 각 숫자와 그 인덱스를 연관시켜야 한다.

```cpp
vector<int> twoSum(vector<int>& nums, int target) {
    unordered_map<int, int> dic;
    for (int i = 0; i < nums.size(); i++) {
        int num = nums[i];
        int complement = target - num;
        if (dic.find(complement) != dic.end()) {
            return {i, dic[complement]};
        }
        
        dic[num] = i;
    }
    
    return {-1, -1};
}
```

```rs
fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut dic: HashMap<i32, i32> = HashMap::new();

    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;
        if let Some(&complement_index) = dic.get(&complement) {
            return vec![i as i32, complement_index];
        }

        dic.insert(num, i as i32);
    }

    vec![-1, -1]
}
```

> 만약 문제가 쌍의 존재 여부를 나타내는 불리언 값이나 숫자 자체를 반환하도록 요구했다면, 세트를 사용하는 것만으로 충분했을 것이다. 하지만 이 문제는 숫자의 인덱스를 요구하므로, 숫자들이 어디에 위치하는지 기억하기 위해 해시 맵을 사용해야 한다.
{: .prompt-tip }

해시 맵 작업이 $O(1)$이므로 전체 시간 복잡도는 $O(n)$이다. 이 해법은 또한 $O(n)$의 공간을 사용한다. 해시 맵이 저장하는 키의 수가 입력 크기와 비례하여 증가하기 때문이다.

---

## **예제 2: 두 번 나타나는 첫 번째 글자 찾기**

> [문제 링크](https://leetcode.com/problems/first-letter-to-appear-twice/)
>
> 문자열 `s`가 주어질 때, 두 번째로 나타나는 첫 번째 문자를 반환해보자. 입력 문자열에는 반드시 중복 문자가 존재한다는 것이 보장된다.
{: .prompt-tip }

가장 간단한 해결 방법은 문자열을 순회하며, 각 문자 `c`에 대해 그 이전까지의 문자들을 다시 검사하면서 일치하는 것이 있는지 확인하는 것이다.

```cpp
// bruce_force

char repeatedCharacter(string s) {
    for (int i = 0; i < s.size(); i++) {
        char c = s[i];
        for (int j = 0; j < i; j++) {
            if (s[j] == c) {
                return c;
            }
        }
    }

    return ' ';
}
```

```rs
// bruce_force

fn repeated_character(s: &str) -> char {
    for (i, c) in s.chars().enumerate() {
        for j in s[..i].chars() {
            if j == c {
                return c;
            }
        }
    }

    ' '
}
```

이 방법은 중첩된 루프 때문에 $O(n^2)$의 시간 복잡도를 갖는다. 하지만, 두 번째 루프는 문자 `c`의 존재 여부를 확인하는 것으로, 세트를 사용하면 $O(1)$에 할 수 있다.

### 예제 2 상세 설명

```cpp
char repeatedCharacter(string s) {
    unordered_set<char> seen;
    for (char c: s) {
        if (seen.find(c) != seen.end()) {
            return c;
        }
        seen.insert(c);
    }

    return ' ';
}
```

```rs
fn repeated_character(s: &str) -> char {
    let mut seen: HashSet<char> = HashSet::new();

    for c in s.chars() {
        if seen.contains(&c) {
            return c;
        }

        seen.insert(c);
    }

    ' '
}
```

이 방법을 사용하면 각 for 문의 반복이 상수 시간에 실행되므로 시간 복잡도는 $O(n)$으로 개선된다.

공간 복잡도에 관한 논의는 더 흥미롭다. 많은 사람들은 입력값이 영어 알파벳 문자만을 포함할 수 있으므로 공간 복잡도가 $O(1)$이라고 주장한다. 이는 문자열 문제에서 자주 볼 수 있는 관점이며 기술적으로 정확하다. 면접 환경에서는 이러한 대답이 안전할 수 있으나, 입력에 허용되는 문자의 수인 $m$에 대해 공간 복잡도가 $O(m)$이 될 수도 있다는 점을 지적하는 것이 좋다. 이는 더 일반적인 관점이며 기술적으로도 정확한 대답이다.

---

## **예제 3: 주어진 정수 배열에서 특정 조건을 만족하는 숫자 찾기**

> 정수 배열 `nums`가 주어졌을 때, `nums`에 `x + 1`도 없고 `x - 1`도 없는 모든 숫자 `x`를 찾아보자.
{: .prompt-tip }

### 예제 3 상세 설명

이 문제는 간단한 방법으로 해결할 수 있다. `nums`를 순회하면서 각 숫자 `x`에 대해 `x + 1` 또는 `x - 1`이 `nums`에 있는지 확인한다. `nums`를 미리 세트로 변환하면, 이러한 확인은 $O(1)$에 수행할 수 있다. 입력을 미리 세트로 변환하는 것은 사전 처리의 한 예이다.

```cpp
vector<int> findNumbers(vector<int>& nums) {
    vector<int> ans;
    unordered_set<int> numsSet(nums.begin(), nums.end());
    
    for (int num: nums) {
        if (numsSet.find(num + 1) == numsSet.end() && numsSet.find(num - 1) == numsSet.end()) {
            ans.push_back(num);
        }
    }
    
    return ans;
}
```

```rs
fn find_numbers(nums: Vec<i32>) -> Vec<i32> {
    let mut ans: Vec<i32> = Vec::new();
    let nums_set: HashSet<i32> = nums.iter().cloned().collect();

    for &num in &nums {
        if !nums_set.contains(&(num + 1)) && !nums_set.contains(&(num - 1)) {
            ans.push(num);
        }
    }

    ans
}
```

이 방법은 각 확인 작업이 $O(1)$이므로, 전체 시간 복잡도는 $O(n)$이 된다. 각 for 문의 반복이 상수 시간에 실행되기 때문이다. 이와 함께, 세트는 $O(n)$의 공간을 차지한다.

---

`if ... in ...` 구문을 사용하는 알고리즘이 있다면, 이러한 연산을 $O(1)$에 실행하기 위해 해시 맵이나 세트를 사용하여 요소를 저장하는 것을 고려해볼 수 있다. 여기서 배운 내용을 활용해 다음 연습 문제들을 풀어보자.

---

## **보너스 문제**

- [1832. Check if the Sentence Is Pangram](https://leetcode.com/problems/check-if-the-sentence-is-pangram/)
- [268. Missing Number](https://leetcode.com/problems/missing-number/)
- [1426. Counting Elements](https://leetcode.com/problems/counting-elements/)
- [217. Contains Duplicate](https://leetcode.com/problems/contains-duplicate/)
- [1436. Destination City](https://leetcode.com/problems/destination-city/)
- [1496. Path Crossing](https://leetcode.com/problems/path-crossing/)

---

> 출처: [Leetcode](https://leetcode.com/explore/interview/card/leetcodes-interview-crash-course-data-structures-and-algorithms/705/hashing/4511/)
{: .prompt-info }
