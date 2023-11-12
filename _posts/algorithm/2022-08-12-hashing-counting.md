---
title: "[DSA] 카운팅"
categories: [Algorithm 연구소]
tags: [Algorithm, DSA, Hashing]
date: 2022-08-12 01:30
math: true
img_path: /assets/img/algorithm/
---

---

![Title](algorithm_title.png)

---

## **개요**

해시 맵을 활용한 카운팅은 일반적인 패턴 중 하나이다. 여기서 "카운팅"이란 다양한 항목들의 발생 빈도를 추적하는 것을 말한다. 이는 우리의 해시 맵이 키를 정수 값에 대응시키게 됨을 의미한다. 어떤 것을 세어야 할 필요가 있을 때, 그 작업을 위해 해시 맵을 사용하는 것을 고려해 볼 수 있다.

슬라이딩 윈도우를 살펴보면, 일부 문제들이 윈도우 내 특정 요소의 수를 제한하는 것을 제약 조건으로 삼는 것을 알 수 있다. 예를 들어, 최대 `k`개의 `0`을 포함하는 가장 긴 부분 문자열 문제가 있다. 이러한 문제에서는 오직 하나의 요소(`0`)에만 집중하고 있기 때문에 간단히 정수 변수 `curr`을 사용할 수 있었다. 반면, 해시 맵은 여러 요소를 포함하는 제약 조건이 있는 문제들을 해결할 수 있는 새로운 방법을 제공한다. 이제 해시 맵을 활용하는 슬라이딩 윈도우 예시를 살펴보며 시작해보자.

---

## **예제 1: 최대 k개의 서로 다른 문자를 포함하는 가장 긴 부분 문자열의 길이**

> 문자열 `s`와 정수 `k`가 주어졌을 때, **최대** `k`개의 서로 다른 문자를 포함하는 가장 긴 부분 문자열의 길이를 찾아보자.
>
> 예를 들어, `s = "eceba"`와 `k = 2`가 주어지면, `3`이라는 결과를 반환한다. 여기서 최대 `2`개의 서로 다른 문자를 가진 가장 긴 부분 문자열은 `"ece"`이다.
{: .prompt-tip }

이 문제는 부분 문자열을 다루고, 부분 문자열에 최대 `k`개의 서로 다른 문자라는 제약 조건이 있다. 이러한 특성으로 인해, 슬라이딩 윈도우 기법을 고려해 볼 필요가 있다. 슬라이딩 윈도우는 제약 조건을 위반할 때까지 윈도우를 오른쪽으로 확장하고, 위반하면 윈도우를 왼쪽으로 줄여나가는 방식으로 작동한다. 이 문제에서는 윈도우 내의 서로 다른 문자 수에 집중한다. 이 제약 조건을 확인하는 전통적인 방법은 매번 전체 윈도우를 검사하는 것인데, 이는 $O(n)$의 시간이 소요될 수 있다. 해시 맵을 사용하면, 이 작업을 $O(1)$의 시간에 수행할 수 있다.

윈도우 내 문자의 빈도수를 추적하기 위해 해시 맵 `counts`를 사용할 수 있다. 이는 문자를 빈도수에 매핑하는 것을 의미한다. 어느 시점에서든 `counts`의 길이(즉, 키의 수)는 서로 다른 문자의 수를 나타낸다. 왼쪽에서 요소를 제거할 때, 해당 요소의 빈도수를 감소시킬 수 있다. 빈도수가 `0`이 되면, 해당 문자는 더 이상 윈도우의 일부가 아니므로, 해당 키를 삭제할 수 있다.

### 예제 1 상세 설명

슬라이딩 윈도우 알고리즘에 대해 잊었다면, 관련 문서를 다시 확인하기 바란다.

이 문제에서 제약 조건은 "윈도우 내에 있는 고유한 문자의 수"이다. 수치적 제한은 `k` 이하이다. 윈도우 내 각 문자의 빈도를 추적하는 해시 맵 `counts`를 사용할 수 있다. `counts`의 길이는 키의 수이며, 이는 제약 조건을 나타낸다. 따라서 `counts`의 길이가 `k`보다 클 때, 윈도우는 유효하지 않은 상태가 된다.

문자 `s[right]`를 추가할 때, counts 내에서 해당 문자의 빈도를 하나 증가시킨다. 만약 `counts` 내에 해당 문자가 없다면, 새로운 키-값 쌍인 `s[right]: 1`을 삽입한다.

문자 `s[left]`를 제거할 때, `counts` 내에서 해당 문자의 빈도를 하나 감소시킨다. 빈도가 `0`이 되면, 해당 문자는 더 이상 존재하지 않는 것이므로, 해시 맵에서 해당 키를 삭제하고 `counts`의 길이를 줄인다.

윈도우의 길이는 `right - left + 1`로 계산된다는 점을 기억하자.

```cpp
int findLongestSubstring(string s, int k) {
    unordered_map<char, int> counts;
    int left = 0, ans = 0;
    
    for (int right = 0; right < s.size(); right++) {
        counts[s[right]]++;
        while (counts.size() > k) {
            counts[s[left]]--;
            if (counts[s[left]] == 0) {
                counts.erase(s[left]);
            }
            left++;
        }
        
        ans = max(ans, right - left + 1);
    }
    
    return ans;
}
```

```rs
fn find_longest_substring(s: &str, k: usize) -> usize {
    let mut counts: HashMap<char, i32> = HashMap::new();
    let (mut left, mut ans) = (0, 0);

    for (right, c) in s.chars().enumerate() {
        *counts.entry(c).or_insert(0) += 1;

        while counts.len() > k {
            let left_char = s.chars().nth(left).unwrap();
            *counts.get_mut(&left_char).unwrap() -= 1;

            if counts[&left_char] == 0 {
                counts.remove(&left_char);
            }

            left += 1;
        }

        ans = ans.max(right - left + 1);
    }

    ans
}
```

이 코드에서 볼 수 있듯이, 해시 맵을 활용하여 원하는 키의 빈도를 저장함으로써, 여러 요소에 대한 제약을 포함하는 슬라이딩 윈도우 문제를 해결할 수 있다. 이전에 살펴본 바와 같이, 각 for 문 반복마다 수행되는 작업이 일정한 시간 내에 처리되면, 슬라이딩 윈도우 문제의 시간 복잡도는 $O(n)$이 된다. 이 경우 해시 맵이 $O(1)$의 연산 시간을 가지고 있기 때문에 해당 조건을 만족한다. 해시 맵은 $O(k)$의 공간을 차지하며, $k$를 초과하는 경우 알고리즘은 해시 맵에서 해당 요소를 제거한다.

---

## **예제 2: 여러 배열의 교집합**

> [문제 링크](https://leetcode.com/problems/intersection-of-multiple-arrays/)
>
> 서로 다른 정수들로 이루어진 `n`개의 배열을 포함한 2차원 배열 `nums`가 주어질 때, 모든 배열에 공통으로 나타나는 숫자들을 포함한 정렬된 `n`개의 배열을 반환해보자.
>
> 예를 들어, `nums = [[3,1,2,4,5],[1,2,3,4],[3,4,5,6]]`이 주어진다면, `[3, 4]`를 반환한다. `3`과 `4`는 모든 배열에 공통으로 나타나는 유일한 숫자들이다.
{: .prompt-tip }

이 문제에서 각 배열은 **서로 다른** 정수들을 포함한다고 한다. 이는 특정 숫자가 `n`번 나타난다면 그 숫자가 모든 배열에 나타나고 있음을 의미한다.

해시 맵 `counts`를 활용하여 요소들의 빈도를 세어보자. 각 내부 배열을 순회하며 `counts`를 각 요소로 업데이트한다. 모든 배열을 순회한 후, 해시 맵을 통해 `n`번 나타나는 숫자들을 찾아낼 수 있다.

### 예제 2 상세 내용

```cpp
vector<int> intersection(vector<vector<int>>& nums) {
    unordered_map<int, int> counts;
    for (vector<int>& arr: nums) {
        for (int x: arr) {
            counts[x]++;
        }
    }
    
    int n = int(nums.size());
    vector<int> ans;
    for (auto [key, val]: counts) {
        if (val == n) {
            ans.push_back(key);
        }
    }
    
    sort(ans.begin(), ans.end());
    return ans;
}
```

```rs
fn intersection(nums: Vec<Vec<i32>>) -> Vec<i32> {
    let mut counts = HashMap::new();

    for arr in nums.iter() {
        for &x in arr {
            *counts.entry(x).or_insert(0) += 1;
        }
    }

    let n = nums.len() as i32;
    let mut ans: Vec<i32> = counts.into_iter()
        .filter(|&(_key, val)| val == n)
        .map(|(key, _val)| key)
        .collect();

    ans.sort_unstable();
    ans
}
```

해시 맵을 사용하는 것이 편리한 이유가 이 문제에서 잘 드러난다. 키가 정수인데 왜 배열 대신 해시 맵을 사용하지 않는지 의문을 가질 수 있다. 배열을 사용할 수도 있지만, 배열은 최대 요소 크기만큼 커야 한다. 예를 들어, `[1, 2, 3, 1000]` 같은 테스트 케이스가 있다면, `1000` 크기의 배열을 초기화해야 하며, 실제 사용되는 인덱스는 소수에 불과하다. 따라서 배열을 사용하면 공간이 크게 낭비될 수 있다. 해시 맵은 때때로 오버헤드가 있을 수 있지만, 전반적으로 더 안전하다. 입력에 `99999999999` 같은 큰 숫자가 있어도 해시 맵은 다른 요소처럼 적절히 처리한다.

$n$개의 리스트가 있고 각 리스트에 평균적으로 $m$개의 요소가 있다고 가정하자. 해시 맵을 채우는 데는 모든 요소를 순회하는 데 $O(n \cdot m)$의 시간이 소요된다. `ans`에 최대 $m$개의 요소가 들어갈 수 있으므로, 최악의 경우 정렬하는 데는 $O(m \cdot \log m)$의 시간이 든다. 따라서 시간 복잡도는 $O(n \cdot m + m \cdot \log m) = O(m \cdot (n + \log m))$이다. 입력의 모든 요소가 고유하다면, 해시 맵은 $n \cdot m$ 크기로 증가하므로, 알고리즘의 공간 복잡도는 $O(n \cdot m)$이다.

---

## **예제 3: 모든 문자가 동일한 횟수로 발생하는지 확인하기**

> [문제 링크](https://leetcode.com/problems/check-if-all-characters-have-equal-number-of-occurrences/)
>
> 문자열 `s`가 주어졌을 때, 모든 문자가 동일한 빈도로 나타나는지 확인해보자.
>
> 예를 들어, `s = "abacbc"`가 주어지면 모든 문자가 두 번 나타므로 true를 반환한다. 반면, `s = "aaabb"`가 주어지면 `"a"`는 3번, `"b"`는 2번 나타나기 때문에 false를 반환한다(`3 != 2`).
{: .prompt-tip }

해시 맵과 세트의 지식을 활용하면, 이 문제는 간단하게 해결할 수 있다. 해시 맵 `counts`를 사용하여 모든 문자의 빈도를 세어본다. `s`를 순회하며 모든 문자의 빈도를 얻는다. 그런 다음 모든 빈도가 동일한지 확인한다.

세트는 중복을 무시하기 때문에, 모든 빈도를 세트에 넣고 길이가 `1`인지 확인하면 모든 빈도가 같은지 검증할 수 있다.

### 예제 3 상세 설명

세트는 빈도를 무시한다는 사실을 기억하자. 같은 요소를 세트에 100번 추가해도 첫 번째 작업만 요소를 추가하고 나머지는 아무 영향을 미치지 않는다.

이 문제에서는 하나의 고유 빈도만 존재하는지 판단하려고 한다. 먼저 해시 맵을 사용하여 각 문자의 빈도를 세어본다. 세는 작업이 끝난 후, 해시 맵의 값들은 우리가 찾는 빈도가 된다.

만약 하나의 고유 빈도만 있다면, 모든 값을 세트에 추가한 후 세트의 길이는 1이 될 것이다. 다른 빈도를 가진 문자가 있다면 세트의 길이는 1보다 커질 것이다. 이는 세트가 모든 고유 빈도를 포함하게 되기 때문이다.

```cpp
bool areOccurrencesEqual(string s) {
    unordered_map<char, int> counts;
    for (char c: s) {
        counts[c]++;
    }
    
    unordered_set<int> frequencies;
    for (auto [key, val]: counts) {
        frequencies.insert(val);
    }
    
    return frequencies.size() == 1;
}
```

```rs
fn are_occurrences_equal(s: &str) -> bool {
    let mut counts: HashMap<char, i32> = HashMap::new();

    for c in s.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }

    let frequencies: HashSet<&i32> = counts.values().collect();

    frequencies.len() == 1
}
```

`s`의 길이를 $n$이라고 할 때, 해시 맵을 채우는 데 $O(n)$의 비용이 든다. 그 다음, 해시 맵의 값들을 세트로 변환하는 데 $O(n)$이 소요된다. 따라서 시간 복잡도는 $O(n)$이다. 해시 맵과 세트가 차지하는 공간은 고유 문자의 수에 해당한다. 이전에 논의된 바와 같이, 일부 사람들은 영어 알파벳에서 유래한 문자들이기 때문에 이를 $O(1)$이라고 주장한다. 보다 일반적인 대답은 공간 복잡도가 $O(k)$라고 말하는 것이다. 여기서 $k$는 입력에 포함될 수 있는 문자의 수로, 이 문제에서는 26이다.

---

## **"정확한" 제약 조건을 가진 부분 배열의 수 계산하기**

우리는 이전 슬라이딩 윈도우 관련 글에서, "제약 조건에 맞는 부분 배열/부분 문자열의 수를 찾는 패턴"에 대해 논의했다. 이런 유형의 문제에서는 `left`와 `right` 사이의 윈도우가 제약 조건을 만족한다면, `left < x <= right` 범위 내의 모든 `x`에서 `right`까지의 윈도우도 같은 제약 조건을 만족한다.

이번에는 더 엄격한 제약 조건을 가진 문제들을 살펴볼 것이다. 이전에 언급된 속성이 반드시 참일 필요는 없다.

> 예를 들어, "**양수만 있는 입력**에 대해 k보다 작은 합을 가진 부분 배열의 수를 찾는" 문제는 슬라이딩 윈도우로 해결할 수 있다. 이 섹션에서는 "**정확히** `k`와 같은 합을 가진 부분 배열의 수를 찾는" 같은 질문을 다룰 것이다.
{: .prompt-tip }

처음에 이 문제들은 매우 어려워 보일 수 있다. 하지만 이 패턴을 배우고 나면 매우 간단해지며, 이 패턴에 속하는 각 문제의 코드가 얼마나 유사한지 알 수 있다. 알고리즘을 이해하기 위해서는 구간 합의 개념을 다시 생각해볼 필요가 있다.

구간 합을 사용하면, 두 구간 합의 차이로 부분 배열의 합을 찾을 수 있다. `k`와 정확히 같은 합을 가진 부분 배열을 찾고자 한다고 가정해보자. 또한 입력의 구간 합을 가지고 있다고 하자. 구간 합에서 `k`와 같은 차이가 나타나는 모든 경우가 `k`와 같은 합을 가진 부분 배열을 나타낸다. 그렇다면 이러한 차이들을 어떻게 찾을까?

먼저 구간 합이 얼마나 자주 발생하는지 매핑하는 해시 맵 `counts`를 선언한다. 이 해시 맵은 구간 합을 얼마나 자주 발생하는지 기록한다(입력에 음수가 있는 경우, 숫자가 구간 합에서 여러 번 나타날 수 있다). `counts[0] = 1`로 초기화해야 한다. 이는 빈 구간 `[]`의 합이 `0`이기 때문이다.

다음으로, 답을 저장할 변수와 `curr`를 선언한다. 입력을 순회하면서, `curr`는 지금까지 순회한 모든 요소들의 합을 나타낼 것이다.

입력을 순회하는 동안, 각 요소에서 `curr`를 업데이트하고 `counts`도 함께 업데이트한다. `counts`를 업데이트하기 전에 먼저 답을 업데이트해야 한다.

답을 어떻게 업데이트할까? 슬라이딩 윈도우 관련 글에서 "부분 배열의 수"를 찾을 때, 각 인덱스에 집중하고 현재 인덱스에서 **끝나는** 유효한 부분 배열이 얼마나 있는지 파악했다. 여기서도 같은 방식을 사용한다. 인덱스 `i`에 도달했다고 가정하자. 이 시점까지 `curr`는 `i`까지의 모든 요소들의 구간 합을 저장하고 있다. 또한 `i` 이전의 모든 다른 구간 합들을 `counts`에 저장했다. 따라서 `i`에서 끝나는 `k`와 같은 합을 가진 부분 배열이 있다면, `curr - k`는 이전에 **이미** 나타났어야 한다.

구간 합의 차이로 부분 배열의 합을 찾는다는 것을 기억하자. 만약 `curr - k`가 이 시점 이전에 구간 합으로 존재했다면, 현재 구간 합이 `curr`일 때, 이 두 구간 합의 차이는 `curr - (curr - k) = k`로, 우리가 찾고자 하는 정확한 값이 된다.

따라서 답은 `counts[curr - k]`로 증가한다. 만약 `curr - k` 구간이 이전에 여러 번 나타났다면, 각각의 구간을 시작점으로 사용하여 현재 인덱스에서 끝나는 `k` 합을 가진 부분 배열을 형성할 수 있다. 그렇기 때문에 빈도를 추적하는 것이 중요하다.

---

## **예제 4: 합이 K인 부분 배열의 수 계산하기**

> [문제 링크](https://leetcode.com/problems/subarray-sum-equals-k/)
>
> 정수 배열 `nums`와 정수 `k`가 주어졌을 때, 합이 `k`인 부분 배열의 수를 계산해보자.
{: .prompt-tip }

이 문제를 이해하기 위해 예시를 통해 알고리즘을 살펴보자. 예를 들어, `nums = [1, 2, 1, 2, 1], k = 3`인 경우를 생각해보자. 합이 `3`인 부분 배열은 총 네 개가 있다 - `[1, 2]`가 두 번, `[2, 1]`이 두 번이다.

이 입력에 대한 구간 합은 순회하는 동안 `curr`가 나타내는 것으로, `[1, 3, 4, 6, 7]`이다. 이 배열에서 `3`이라는 차이는 세 번 나타난다: `(4 - 1)`, `(6 - 3)`, `(7 - 4)`.

하지만 유효한 부분 배열이 네 개라고 했는데 어떻게 이해할 수 있을까? 해시 맵을 `0: 1`로 초기화해야 한다는 것을 기억해야 한다. 이는 `k`와 같은 합을 가진 구간이 있다면, `0: 1`을 초기화하지 않으면 `curr - k = 0`이 해시 맵에 나타나지 않아 유효한 부분 배열을 놓칠 수 있기 때문이다.

따라서 인덱스 1, 2, 3, 4에서 `curr - k`가 이전에 나타났음을 알 수 있다. 모든 요소가 양수이므로 `curr - k` 값은 각각 한 번씩만 나타난다. 그러므로 답은 `4`가 된다.

### 예제 4 상세 설명

```cpp
int subarraySum(vector<int>& nums, int k) {
    unordered_map<int, int> counts;
    counts[0] = 1;
    int ans = 0, curr = 0;
    
    for (int num: nums) {
        curr += num;
        ans += counts[curr - k];
        counts[curr]++;
    }
    
    return ans;
}
```

```rs
fn subarray_sum(nums: &[i32], k: i32) -> i32 {
    let mut counts = HashMap::new();

    counts.insert(0, 1);
    let (mut ans, mut curr) = (0, 0);

    for &num in nums {
        curr += num;
        ans += counts.get(&(curr - k)).unwrap_or(&0);
        *counts.entry(curr).or_insert(0) += 1;
    }

    ans
}
```

`curr`가 계속 증가한다면 해시 맵의 값은 1을 넘지 않으므로 세트만 사용하면 되지 않을까 생각할 수 있다. 하지만 제약 조건은 `-1000 <= nums[i] <= 1000`이다. 예를 들어, `nums = [1, -1, 1, -1], k = 0`인 경우를 살펴보자. 네 개의 유효한 부분 배열이 있다: `[1, -1]` 두 번, `[-1, 1]` 한 번, 전체 배열 `[1, -1, 1, -1]`.

구간 합은 `[1, 0, 1, 0]`이다. 마지막 인덱스에서 끝나는 두 개의 부분 배열이 있다 - `[1, -1]`과 전체 배열. `counts[0] = 1`로 초기화했기 때문에, 두 번째 인덱스 이후에는 `counts[0] = 2`가 된다. 따라서 마지막 인덱스에 도달하고 `ans += counts[curr - k] = ans += counts[0]`를 수행하면, 두 부분 배열을 답에 추가한다. 입력에 음수가 포함되어 있을 때, **동일한 구간 합이 여러 번** 나타날 수 있으며, 빈도를 계산하기 위해 해시 맵이 필요하다.

요약하자면:

- `curr`를 사용하여 구간 합을 추적한다.
- 어떤 인덱스 `i`에서, `i`까지의 합은 `curr`이다. 만약 인덱스 `j`의 구간 합이 `curr - k`라면, `j + 1`부터 `i`까지의 부분 배열의 합은 `curr - (curr - k) = k`가 된다.
- 배열에 음수가 있을 수 있기 때문에, 동일한 구간 합이 여러 번 나타날 수 있다. 해시 맵 `counts`를 사용하여 구간 합이 몇 번 나타났는지 추적한다.
- 모든 인덱스 `i`에서, `curr - k`의 빈도는 `k`와 같은 합을 가진 부분 배열의 수와 같으며, 이는 `i`에서 끝난다. 이를 답에 더한다.

이 알고리즘의 시간 복잡도와 공간 복잡도는 모두 $O(n)$이다. 여기서 $n$은 `nums`의 길이이다. 각 for 문 반복은 상수 시간에 실행되며, 해시 맵은 최대 $n$개의 요소로 성장할 수 있다.

---

## **예제 5: 좋은 부분 배열의 수 세기**

> [문제 링크](https://leetcode.com/problems/count-number-of-nice-subarrays/)
>
> 양의 정수 배열 `nums`와 정수 `k`가 주어졌을 때, 정확히 `k`개의 홀수를 포함하는 부분 배열의 수를 찾아보자.
>
> 예를 들어, `nums = [1, 1, 2, 1, 1], k = 3`인 경우, 답은 `2`개이다. `3`개의 홀수를 포함하는 부분 배열은 `[1, 1, 2, 1, 1]`과 `[1, 1, 2, 1, 1]` 두 가지이기 때문이다.
{: .prompt-tip }

이전 예제에서 제약 조건은 합이었고, `curr`은 구간 합을 나타냈다. 이 문제에서는 제약 조건이 홀수의 개수이므로, `curr`은 **홀수의 수**를 추적한다. 각 요소에서 `curr - k`를 다시 조회할 수 있다. 예제 테스트 케이스에서, 마지막 인덱스에서 `curr = 4`이다. 이는 배열에 `4`개의 홀수가 있음을 의미한다. 첫 번째 인덱스에서 `curr = 1`이다. `4 - 1 = 3`이고, 인덱스 `1`에서 `4`까지의 부분 배열은 답 중 하나이다(`[1, 1, 2, 1, 1]`).

> 수가 홀수인지 확인하기 위해서는 해당 수를 2로 나눈 나머지를 취하면 된다. `x`가 홀수라면, `x % 2 = 1`이다.
{: .prompt-tip }

### 예제 5 상세 설명

슬라이딩 윈도우를 사용하지 않지만, "유효한" 부분 배열을 찾는 것에는 같은 아이디어를 적용할 수 있다.

이전 예제에서 제약 조건은 "부분 배열의 합"이었고, 수치적 제한은 `k`와 같아야 했다.

이 문제에서는 제약 조건이 "부분 배열 내 홀수의 개수"이며, 수치적 제한도 `k`와 같다.

여기서 구간 합을 좀 더 창의적으로 활용해야 한다. `curr`을 구간 합으로 사용하는 대신, 현재 구간에서 관찰된 홀수의 수를 추적한다. 이를 "구간 홀수 개수"라고 부를 수 있다. 만약 `curr - k`가 존재한다면, 이전에 `curr - k`개의 홀수를 가진 구간이 있었다는 뜻이다. 현재 구간에는 `curr`개의 홀수가 있다. 이 두 구간 사이의 차이는 구간 사이의 홀수 개수를 나타내며, `curr - (curr - k) = k`개의 홀수가 된다.

수가 홀수인 경우 2로 나눈 나머지가 `1`이 된다. 그렇지 않으면 `0`이 된다. 따라서 각 수에 대해 `curr += num % 2`를 수행할 수 있다.

여기서부터 모든 것이 완전히 동일하게 작동한다. 실제로, 두 문제의 코드를 비교해보면 차이는 `"% 2"` 두 글자뿐이다. 다음 보너스 문제들을 직접 풀어보며 이 글에서 배운 지식을 적용해보자.

```cpp
int numberOfSubarrays(vector<int>& nums, int k) {
    unordered_map<int, int> counts;
    counts[0] = 1;
    int ans = 0, curr = 0;
    
    for (int num: nums) {
        curr += num % 2;
        ans += counts[curr - k];
        counts[curr] += 1;
    }
    
    return ans;
}
```

```rs
fn number_of_subarrays(nums: &[i32], k: i32) -> i32 {
    let mut counts = HashMap::new();
    counts.insert(0, 1);
    let (mut ans, mut curr) = (0, 0);

    for &num in nums {
        curr += num % 2;
        ans += *counts.get(&(curr - k)).unwrap_or(&0);
        *counts.entry(curr).or_insert(0) += 1;
    }

    ans
}
```

"이 패턴에 속하는 각 문제의 코드가 얼마나 비슷한지 알 수 있을 것"이라고 말씀드렸던 것을 기억하는가? 두 가지 다른 문제와 코드의 차이는 **말 그대로** "`% 2`"라는 두 글자뿐이다. 다음 몇 가지 연습 문제를 직접 풀어보면서 이 글에서 배운 지식을 적용해 보자.

이 알고리즘의 시간 복잡도와 공간 복잡도는 이전 문제와 같다($O(n)$). 그 이유도 동일하다.

---

## **보너스 문제**

- [2225. Find Players With Zero or One Losses](https://leetcode.com/problems/find-players-with-zero-or-one-losses/)
- [1133. Largest Unique Number](https://leetcode.com/problems/largest-unique-number/)
- [1189. Maximum Number of Balloons](https://leetcode.com/problems/maximum-number-of-balloons/)
- [1748. Sum of Unique Elements](https://leetcode.com/problems/sum-of-unique-elements/)
- [1394. Find Lucky Integer in an Array](https://leetcode.com/problems/find-lucky-integer-in-an-array/)
- [1207. Unique Number of Occurrences](https://leetcode.com/problems/unique-number-of-occurrences/)
- [451. Sort Characters By Frequency](https://leetcode.com/problems/sort-characters-by-frequency/)
- [1512. Number of Good Pairs](https://leetcode.com/problems/number-of-good-pairs/)
- [930. Binary Subarrays With Sum](https://leetcode.com/problems/binary-subarrays-with-sum/)
- [1695. Maximum Erasure Value](https://leetcode.com/problems/maximum-erasure-value/)
- [567. Permutation in String](https://leetcode.com/problems/permutation-in-string/)

---

> 출처: [Leetcode](https://leetcode.com/explore/interview/card/leetcodes-interview-crash-course-data-structures-and-algorithms/705/hashing/4512/)
{: .prompt-info }
