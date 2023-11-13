---
title: "[DSA] 해시 맵을 활용한 추가 예제"
categories: [Algorithm 연구소]
tags: [Algorithm, DSA, Hashing]
date: 2022-08-13 02:30
math: true
img_path: /assets/img/algorithm/
---

---

![Title](algorithm_title.png)

---

## **개요**

해시 맵은 매우 다양한 곳에서 사용되며, 이를 알고리즘에 활용하는 방법은 무한히 많다. 해시 맵의 중요성을 감안하여, 다양한 문제에서 해시 맵이 어떻게 사용될 수 있는지 몇 가지 추가 예시를 살펴보겠다. 인터뷰를 통과하기 위해서는 해시 맵에 익숙해져야 한다.

---

## **예제 1: 애너그램 그룹화**

> [문제 링크](https://leetcode.com/problems/group-anagrams/)
>
> 문자열 배열 `strs`가 주어졌을 때, [애너그램](https://en.wikipedia.org/wiki/Anagram)을 함께 그룹화해보자.
>
> 예를 들어, `strs = ["eat", "tea", "tan", "ate", "nat", "bat"]`가 주어졌을 때, 결과는 `[["bat"], ["nat", "tan"], ["ate", "eat", "tea"]]`이다.
{: .prompt-tip }

두 문자열이 서로 애너그램인지를 판별하기 위해, 두 해시 맵을 사용하여 각 문자열의 문자의 빈도를 세고, 해시 맵이 같은지 비교할 수 있다. 그러나 이 방법은 실행하기 까다롭고, 그룹에 문자열이 2개 이상 있을 경우 그룹화하는 데 도움이 되지 않는다. 각 그룹을 고유하게 식별할 수 있는 방법이 필요하다.

두 문자열이 서로 애너그램인지 확인하는 가장 명확한 방법은 두 문자열을 정렬한 후 동일한지를 확인하는 것이다. 또한, 한 그룹에 있는 모든 문자열은 정렬했을 때 동일하므로, 정렬된 버전을 키로 사용할 수 있다. 이 키를 해시 맵에 매핑하여 그룹을 쉽게 구분할 수 있다.

각 그룹은 고유한 "식별자"(정렬된 문자열)를 가지고 있으며, 이를 이용하여 해시 맵에서 쉽게 그룹화할 수 있다.

### 예제 1 상세 설명

한 그룹에 여러 문자열이 포함될 수 있다는 점을 고려하여, 어떤 문자열이 어떤 그룹에 속하는지 쉽게 식별할 방법이 필요하다.

두 문자열이 서로 애너그램이 되려면 정렬되어야 한다. 문자열을 정렬하면 문자가 잘 정의된 순서대로 나타나게 되며, 애너그램은 동일한 문자를 가지므로 이들이 같은 순서로 나타나면 동일하다는 것을 의미한다.

예를 들어, 문자열 `"bcab"`을 정렬하면 `"abbc"`가 된다. 이는 모든 `"bcab"`의 애너그램이 정렬될 때 `"abbc"`가 될 것이므로, 이를 식별자로 사용할 수 있다. 각 문자열의 식별자를 해시 맵의 키로 사용하여 모든 애너그램을 쉽게 그룹화할 수 있다.

```cpp
vector<vector<string>> groupAnagrams(vector<string>& strs) {
    unordered_map<string, vector<string>> groups;
    for (string& s: strs) {
        string t = s;
        sort(t.begin(), t.end());
        groups[t].push_back(s);
    }
    
    vector<vector<string>> ans;
    for (auto [key, val]: groups) {
        ans.push_back(val);
    }
    
    return ans;
}
```

```rs
fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut groups: HashMap<String, Vec<String>> = HashMap::new();

    for s in strs.iter() {
        let mut t = s.clone();
        let mut chars: Vec<char> = t.chars().collect();
        chars.sort_unstable();
        t = chars.into_iter().collect();

        groups.entry(t).or_insert(Vec::new()).push(s.clone());
    }

    groups.into_values().collect()
}
```

`strs`의 길이가 $n$이고 문자열의 평균 길이가 $m$일 때, 각 문자열을 순회하며 정렬하는 데는 $O(n \cdot m \cdot \log m)$의 비용이 든다. 그 후, 키를 순회해야 하는데, 최악의 시나리오에서 일치하는 애너그램이 없을 때 $n$개의 그룹이 있으므로, 이는 $O(n)$의 비용이 들며, 전체 시간 복잡도는 $O(n \cdot m \cdot \log m)$이다(마지막 +$n$은 중요하지 않다). 공간 복잡도는 해시 맵 내의 배열에 각 문자열이 배치되므로 $O(n \cdot m)$이다.

> 이 문제를 해결하는 또 다른 방법은 정렬된 문자열 대신 각 문자의 개수를 나타내는 26자리 튜플을 키로 사용하는 것이다. 이 방법은 문제에 의해 정의된 26이 상수이기 때문에 기술적으로 $O(n \cdot m)$에 문제를 해결할 수 있지만, 문자열이 작은 경우 상수 요소 때문에 더 느려질 수 있다.
>
> 또한 이 방법은 문자열이 26개의 다른 문자만을 가질 수 있다고 가정하는데, 이는 여기서는 타당하지만 일반적으로는 효과적이지 않고, 후속 질문에 대한 대응력이 떨어진다.
{: .prompt-tip }

---

## **예제 2: 뽑아야 하는 최소 연속 카드 수**

> [문제 링크](https://leetcode.com/problems/minimum-consecutive-cards-to-pick-up/)
>
> 정수 배열 `cards`가 주어지면, 적어도 하나의 중복이 포함된 가장 짧은 부분 배열의 길이를 찾아보자. 배열에 중복이 없으면 `-1`을 반환해야 한다.
{: .prompt-tip }

이 문제는 슬라이딩 윈도우 기법으로 해결할 수 있지만, 여기서는 해시 맵에 중점을 둔 다른 접근 방법을 살펴본다. 이 문제는 "같은 요소 사이의 최단 거리는 얼마인가?"로 표현할 수 있다. 배열을 순회하며 해시 맵을 사용해 각 요소의 인덱스를 기록한다. 그런 다음 이 인덱스들을 순회하여 최단 거리를 찾는다. 예를 들어, `cards = [1, 2, 6, 2, 1]`이 주어지면, `1`은 `[0, 4]`, `2`는 `[1, 3]`, `6`은 `[2]`로 매핑한다. 이후 각 값들을 순회하며 `2`를 뽑을 때 최소 차이를 얻는다는 것을 알 수 있다.

### 예제 2 상세 설명

중복을 포함하는 가장 짧은 부분 배열은 첫 번째와 마지막 요소가 중복이어야 한다. 그렇지 않으면 가장자리에서 비용 없이 요소들을 제거할 수 있다. 따라서 같은 요소 사이의 가장 짧은 거리를 찾아야 한다.

배열을 한 번 순회하며 각 요소의 위치를 해시 맵에 기록한다. 해시 맵의 키는 요소이며, 값은 해당 요소가 나타나는 모든 인덱스의 배열이다. 인덱스를 오름차순으로 순회하기 때문에, 해시 맵 내의 각 배열도 오름차순으로 정렬된다.

이제 각 요소를 개별적으로 확인할 수 있다. 최소 거리를 찾기 위해 배열이 정렬되어 있으므로, 인접한 쌍만 확인하면 된다.

```cpp
int minimumCardPickup(vector<int>& cards) {
    unordered_map<int, vector<int>> dic;
    
    for (int i = 0; i < cards.size(); i++) {
        dic[cards[i]].push_back(i);
    }
    
    int ans = INT_MAX;
    for (auto [key, arr]: dic) {
        for (int i = 0; i < arr.size() - 1; i++) {
            ans = min(ans, arr[i + 1] - arr[i] + 1);
        }
    }
    
    return ans == INT_MAX ? -1 : ans;
}
```

```rs
fn minimum_card_pickup(cards: Vec<i32>) -> i32 {
    let mut dic: HashMap<i32, Vec<usize>> = HashMap::new();

    for (i, &card) in cards.iter().enumerate() {
        dic.entry(card).or_default().push(i);
    }

    let mut ans = i32::MAX;
    for arr in dic.values() {
        for window in arr.windows(2) {
            if let [a, b] = window {
                ans = ans.min((b - a) as i32 + 1);
            }
        }
    }

    if ans == i32::MAX { -1 } else { ans }
}
```

이 알고리즘의 시간 복잡도는 중첩된 루프가 있음에도 불구하고 여전히 $O(n)$이다. 이는 루프 안의 루프가 전체적으로 $n$번 이상 돌지 않기 때문이다. 여기서 $n$은 입력된 배열의 길이를 의미한다.

이 알고리즘을 더 효율적으로 만들기 위해, 모든 인덱스를 저장하는 대신 각 숫자에 대해 가장 최근에 본 인덱스만 저장하는 방법이 있다. 이렇게 하면 평균적으로 필요한 저장 공간이 줄어든다. 현재 알고리즘은 늘 $O(n)$ 만큼의 공간을 필요로 하지만, 이 개선 방법을 사용하면 중복이 전혀 없는 최악의 경우에만 $O(n)$의 공간이 필요하다.

```cpp
int minimumCardPickup(vector<int>& cards) {
    unordered_map<int, int> dic;
    int ans = INT_MAX;
    
    for (int i = 0; i < cards.size(); i++) {
        if (dic.find(cards[i]) != dic.end()) {
            ans = min(ans, i - dic[cards[i]] + 1);
        }
        
        dic[cards[i]] = i;
    }
    
    return ans == INT_MAX ? -1 : ans;
}
```

```rs
fn minimum_card_pickup(cards: Vec<i32>) -> i32 {
    let mut dic: HashMap<i32, usize> = HashMap::new();
    let mut ans = i32::MAX;

    for (i, &card) in cards.iter().enumerate() {
        if let Some(&last_index) = dic.get(&card) {
            ans = ans.min(i as i32 - last_index as i32 + 1);
        }
        dic.insert(card, i);
    }

    if ans == i32::MAX { -1 } else { ans }
}
```

이 알고리즘은 반복을 줄임으로써 더 빠르게 실행되지만, 두 알고리즘의 시간 복잡도는 모두 $O(n)$이며, 여기서 $n$은 입력 배열의 길이다.

---

## **예제 3: 자릿수 합이 같은 쌍의 최대 합**

> [문제 링크](https://leetcode.com/problems/max-sum-of-a-pair-with-equal-sum-of-digits/)
>
> 정수 배열 `nums`가 주어졌을 때, `nums[i]`와 `nums[j]`의 **자릿수 합**이 같을 때 `nums[i] + nums[j]`의 최대값을 찾는다. 자릿수 합이 같은 쌍이 없으면 `-1`을 반환해보자.
{: .prompt-tip }

이 문제는 이전에 살펴본 애너그램 그룹화와 유사하다. 첫 번째 예제에서는 정렬된 문자열을 기준으로 그룹을 식별했다. 이 문제에서는 자릿수 합을 기준으로 그룹을 식별할 수 있다. 배열을 순회하며 같은 자릿수 합을 가진 모든 숫자를 해시 맵에 그룹화한 후, 이 해시 맵을 순회하며 적어도 2개의 원소를 가진 각 그룹에서 최대 2개의 원소를 찾아 정렬하여 그 합을 구한다.

### 예제 3 상세 설명

애너그램 그룹화 문제에서는 한 그룹에 여러 다른 문자열이 있을 수 있어, 어떤 문자열이 어떤 그룹에 속하는지 쉽게 식별할 방법이 필요했다. 우리는 정렬된 문자열을 각 문자열의 "식별자"로 사용했다.

이 문제에서는 많은 숫자들이 같은 자릿수 합을 가질 수 있다. 각 숫자에 대해 자릿수 합을 식별자로 간단히 사용할 수 있다. 각 자릿수 합에 대해 그룹 내에서 가장 큰 두 숫자를 찾아 그 합을 구하면 된다.

```cpp
int maximumSum(vector<int>& nums) {
    unordered_map<int, vector<int>> dic;
    for (int num: nums) {
        int digitSum = getDigitSum(num);
        dic[digitSum].push_back(num);
    }
    
    int ans = -1;
    for (auto [key, curr]: dic) {
        if (curr.size() > 1) {
            sort(curr.begin(), curr.end(), greater<int>());
            ans = max(ans, curr[0] + curr[1]);
        }
    }
    
    return ans;
}
    
int getDigitSum(int num) {
    int digitSum = 0;
    while (num > 0) {
        digitSum += num % 10;
        num /= 10;
    }
    
    return digitSum;
}
```

```rs
fn maximum_sum(nums: Vec<i32>) -> i32 {
    let mut dic: HashMap<i32, Vec<i32>> = HashMap::new();
    for &num in &nums {
        let digit_sum = get_digit_sum(num);
        dic.entry(digit_sum).or_default().push(num);
    }

    let mut ans = -1;
    for curr in dic.values() {
        if curr.len() > 1 {
            let mut sorted = curr.clone();
            sorted.sort_by(|a, b| b.cmp(a));
            ans = ans.max(sorted[0] + sorted[1]);
        }
    }

    ans
}

fn get_digit_sum(mut num: i32) -> i32 {
    let mut digit_sum = 0;
    while num > 0 {
        digit_sum += num % 10;
        num /= 10;
    }

    digit_sum
}
```

이 알고리즘은 정렬 때문에 비효율적이며, 입력에 있는 모든 숫자의 자릿수 합이 같을 경우 $O(n \cdot \log{}n)$의 비용이 발생할 수 있다. 여기서 $n$은 입력 배열의 길이다. 이전 문제와 마찬가지로, 그룹의 모든 숫자를 저장할 필요는 없다. 각 자릿수 합에 대해 지금까지 본 가장 큰 숫자만 저장함으로써 시간 복잡도와 평균 공간 복잡도를 개선할 수 있다.

```cpp
int maximumSum(vector<int>& nums) {
    unordered_map<int, int> dic;
    int ans = -1;
    
    for (int num: nums) {
        int digitSum = getDigitSum(num);
        if (dic.find(digitSum) != dic.end()) {
            ans = max(ans, num + dic[digitSum]);
        }
        dic[digitSum] = max(dic[digitSum], num);
    }
    return ans;
}
    
int getDigitSum(int num) {
    int digitSum = 0;
    while (num > 0) {
        digitSum += num % 10;
        num /= 10;
    }
    
    return digitSum;
}
```

```rs
fn maximum_sum(nums: Vec<i32>) -> i32 {
    let mut dic: HashMap<i32, i32> = HashMap::new();
    let mut ans = -1;

    for &num in &nums {
        let digit_sum = get_digit_sum(num);
        let prev_num = dic.get(&digit_sum).unwrap_or(&0);
        ans = ans.max(num + prev_num);

        dic.insert(digit_sum, *prev_num.max(&num));
    }

    ans
}

fn get_digit_sum(mut num: i32) -> i32 {
    let mut digit_sum = 0;
    while num > 0 {
        digit_sum += num % 10;
        num /= 10;
    }

    digit_sum
}
```

마지막 예제와 마찬가지로, 첫 번째 알고리즘은 해시 맵의 값에 모든 요소를 저장하기 때문에 항상 $O(n)$의 공간을 사용하지만, 개선된 방법을 사용하면 평균적으로 훨씬 적은 공간을 사용한다. 왜냐하면 각 키는 단지 정수만 저장하기 때문이다. 또한 추가 반복과 각 반복에서의 정렬을 줄임으로써 시간 복잡도는 $O(n)$이 되며, 여기서 $n$은 입력 배열의 길이다.

---

## **예제 4: 동일한 행과 열 쌍**

> [문제 링크](https://leetcode.com/problems/equal-row-and-column-pairs/)
>
> `n x n` 크기의 행렬 `grid`가 주어졌을 때, `R`이 행이고 `C`가 열일 경우, `R`과 `C`를 1차원 배열로 간주했을 때, 동일한 `(R, C)` 쌍의 수를 반환해보자.
{: .prompt-tip }

동일한 쌍의 수를 어떻게 계산할까? 예를 들어, 세 개의 행이 `[1, 2, 3]`의 형태로 있고, 두 개의 열이 같다고 가정해보자. 세 행 각각이 두 열과 짝을 이루므로, 총 `3 * 2 = 6`쌍이 된다. 각 행이 몇 번 나타나는지 세기 위해 해시 맵을 사용하며, 열에 대해서도 같은 방법으로 두 번째 해시 맵을 사용한다. 그리고 행 해시 맵을 순회하며, 각 행이 열로 나타난 적이 있는지 확인한다. 만약 그렇다면, 나타난 횟수의 곱을 답에 더한다.

배열은 해시 맵의 키로 사용하기 어렵다. 배열은 변경 가능하기 때문이다. 따라서 행과 열을 문자열이나 튜플 같은 해시 가능한 형태로 변환해야 한다. 이는 사용하는 프로그래밍 언어에 따라 최적의 방법이 다를 수 있다.

### 예제 4 상세 설명

만약 `x`개의 행이 `y`개의 열과 동일하다면, `x`개의 행 각각은 `y`개의 열과 짝을 이룰 수 있다. 이는 `x * y` 쌍이 있다는 뜻이다.

행과 열을 별도의 해시 맵으로 관리함으로써, 주어진 행이나 열이 몇 번 행으로, 몇 번 열로 나타났는지 쉽게 찾을 수 있다. 그 후, 그들의 등장 횟수를 곱하여 답에 더한다.

문제는 행과 열을 해시 가능한 형식으로 변환하는 것이다. 파이썬에서는 튜플로 쉽게 변환할 수 있다. 자바와 자바스크립트에서는 쉼표로 구분된 문자열로 변환하며, C++에서는 `map` 데이터 구조를 사용해 변경 가능한 키를 사용할 수 있다.

```cpp
int equalPairs(vector<vector<int>>& grid) {
    // C++ map은 키로 벡터를 받을 수 있다.
    // 그러나 unordered_map 대신 map을 사용하면 O(log n)
    // 연산이지만 여전히 O(n)보다는 훨씬 빠르다.
    map<vector<int>, int> dic;
    for (vector<int> row: grid) {
        dic[row]++;
    }
    
    map<vector<int>, int> dic2;
    for (int col = 0; col < grid[0].size(); col++) {
        vector<int> currentCol;
        for (int row = 0; row < grid.size(); row++) {
            currentCol.push_back(grid[row][col]);
        }
        
        dic2[currentCol]++;
    }
    
    int ans = 0;
    for (auto [arr, val]: dic) {
        ans += val * dic2[arr];
    }
    
    return ans;
}
```

---

그리드의 크기가 $n \cdot n$일 경우, 이 알고리즘의 시간 복잡도는 $O(n^2)$이다. 총 $n^2$개의 요소가 있으며, 각 요소는 처음에 두 번 반복해서 처리된다(한 번은 해당하는 행을 위해, 다른 한 번은 열을 위해). 해시 맵을 만들고 순회하는 과정은 이 초기 반복에 의해 지배된다. 공간 복잡도는 $O(n^2)$이 되는데, 이는 모든 행과 열이 서로 다를 경우 각각의 해시 맵이 $n$ 크기까지 성장하고, 각 키의 길이가 $n$이 될 수 있기 때문이다.

---

## **마무리**

해시 맵과 관련된 문제를 계속 다룰 수도 있지만, 우리는 다른 주제들도 살펴볼 필요가 있다. 해시 맵을 좋아한다면, 앞으로의 과정에서 계속해서 사용할 것이라는 점이 좋은 소식일 것이다. 앞서 언급했듯이, 해시 맵은 거의 모든 알고리즘에 쓰이는 중요한 요소이다. 다음 단계로 넘어가기 전에, 연습 문제들과 해싱 퀴즈를 통해 지식을 시험해 보자.

---

## **보너스 문제**

- [205. Isomorphic Strings](https://leetcode.com/problems/isomorphic-strings/)
- [290. Word Pattern](https://leetcode.com/problems/word-pattern/)
- [791. Custom Sort String](https://leetcode.com/problems/custom-sort-string/)
- [1657. Determine if Two Strings Are Close](https://leetcode.com/problems/determine-if-two-strings-are-close/)

---

> 출처: [Leetcode](https://leetcode.com/explore/interview/card/leetcodes-interview-crash-course-data-structures-and-algorithms/705/hashing/4645/)
{: .prompt-info }
