---
title: "추가 해싱 예제"
categories: [Algorithm 연구소]
tags: [Algorithm]
date: 2023-09-12 02:30
math: true
img_path: /assets/img/algorithm/
---

---

![Title](algorithm_title.png)

---

## **개요**

해시 맵은 다양한 곳에서 활용된다. 몇 가지 일반적인 패턴을 소개했지만, 해시 맵을 알고리즘에 통합할 수 있는 방법은 무수히 많다. 해시 맵의 중요성과 다양한 문제 상황에서 어떻게 활용할 수 있는지 추가적인 예시로 설명하겠다. 코딩 테스트 준비 과정에서 해시 맵에 익숙해지는 것이 중요하다는 점을 강조하고 싶다.

---

> **예제 1**: [Group Anagrams](https://leetcode.com/problems/group-anagrams/)
>
> 문자열 배열 `strs`가 주어질 때, [애너그램](https://en.wikipedia.org/wiki/Anagram)끼리 그룹화하시오.
>
> 예: `strs = ["eat","tea","tan","ate","nat","bat"]`인 경우, `[["bat"],["nat","tan"],["ate","eat","tea"]]`를 반환한다.
{: .prompt-general }

두 문자열이 서로 애너그램인지 어떻게 확인할까? 두 개의 해시 맵을 사용하여 각 문자열의 모든 문자의 빈도를 계산한 다음, 이 두 해시 맵이 동일한지 비교할 수 있다. 그러나 이 방식은 구현이 복잡하며, 그룹에 2개 이상의 문자열이 포함된 경우 문자열을 함께 그룹화하는 데 효율적이지 않다. 그룹을 구분할 수 있는 고유한 방법이 필요하다는 것이다.

두 문자열이 애너그램 관계인지 확인하는 가장 간단한 방법은 두 문자열을 정렬하여 동일한지 비교하는 것이다. 그룹 내의 모든 문자열은 정렬하면 동일하므로, 이 정렬된 문자열을 키로 사용할 수 있다. 이 키를 해시 맵에 매핑하여 그룹을 구분하면, 해시 맵만을 사용해서 답을 도출할 수 있다.

다음은 관련된 C++ 코드이다:

```cpp
class Solution {
public:
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
};
```

> Python 참고: `dictionary.values()`는 실제로 목록을 반환하는 것이 아니라 [뷰 객체](https://stackoverflow.com/questions/8957750/what-are-dictionary-view-objects)를 반환한다. 그러나 LeetCode 에서는 이를 유효한 형식으로 받아들인다.
{: .prompt-general }

`strs`의 길이를 $$n$$, 문자열의 평균 길이를 m이라고 가정하면, 각 문자열을 반복하여 정렬하면 $$O(n \cdot m \cdot \log{}m)$$ 비용이 든다. 그런 다음 키를 반복해야 한다. 최악의 경우, 일치하는 애너그램이 없을 때 n개의 그룹이 발생하므로 $$O(n)$$이 소요된다. 따라서 전체 시간 복잡도는 $$O(n \cdot m \cdot \log{}m)$$이며, 공간 복잡도는 $$O(n \cdot m)$$이다.

> 이 문제를 해결하는 또 다른 방법은 정렬된 문자열 대신 각 문자의 빈도를 나타내는 길이 26의 튜플을 키로 사용하는 것이다. 이 경우 26은 문제에 의해 주어진 상수이므로 $$O(n \cdot m)$$의 시간 복잡도로 문제를 해결할 수 있다. 그러나 문자열이 작은 경우, 빅오 표기법에서 숨겨진 상수 계수 때문에 실제 실행 시간이 느려질 수 있다.
>
> 이 방법의 한계는 문자열에 26개의 알파벳 문자만 포함될 수 있다는 가정을 기반으로 한다는 것이다. 이는 일반적인 상황에는 적용되지 않을 수 있으며, 특정 상황에서는 부적절하다.
{: .prompt-general }

---

> **예제 2**: [받을 수 있는 최소 연속 카드의 수](https://leetcode.com/problems/minimum-consecutive-cards-to-pick-up/)
>
> 정수 배열 `cards`가 주어졌을 때, 적어도 하나의 중복을 포함하는 가장 짧은 하위 배열의 길이를 구한다. 배열에 중복이 없으면 `-1`을 반환한다.
{: .prompt-general }

슬라이딩 윈도우를 사용하여 이 문제를 해결할 수 있지만, 여기서는 해시 맵을 더 강조하면서 다른 접근 방식을 살펴보겠다. 이 문제는 같은 요소 두 개 사이의 최단 거리는 얼마인가라는 질문과 동일하다. 배열을 검사하며 해시 맵을 사용하여 모든 요소의 인덱스를 기록한다. 그러면 해당 인덱스를 반복하여 최단 거리를 찾을 수 있다. 예를 들어 `cards = [1, 2, 6, 2, 1]`이 주어진다면 `1: [0, 4]`, `2: [1, 3]`, `6: [2]`와 같이 매핑할 수 있다. 그런 다음 값을 반복하여 `2`의 인덱스 사이에서 최소 차이를 얻을 수 있다는 것을 확인할 수 있다.

```cpp
class Solution {
public:
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
};
```

알고리즘에 중첩 루프가 있지만, 시간 복잡도는 여전히 $$O(n)$$이다. 이는 중첩 루프의 내부 루프가 배열의 요소 인덱스를 반복하기 때문이다.

모든 인덱스를 저장하는 대신 각 숫자에 대해 가장 최근에 본 인덱스만 저장함으로써 이 알고리즘을 더 효율적으로 만들 수 있다. 이렇게 하면 평균 공간 복잡도가 줄어든다.

```cpp
class Solution {
public:
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
};
```

두 알고리즘의 시간 복잡도는 $$O(n)$$이다. 여기서 $$n$$은 입력 배열의 길이이다.

---

> **예제 3**: [자릿수 합이 같은 쌍의 최대 합계](https://leetcode.com/problems/max-sum-of-a-pair-with-equal-sum-of-digits/)
>
> 정수의 배열 `nums`가 주어졌을 때, `nums[i]`와 `nums[j]`의 자릿수 합이 같을 경우의 `nums[i] + nums[j]`의 최대값을 구한다. **자릿수 합**이 같은 숫자 쌍이 없으면 `-1`을 반환한다.
{: .prompt-general }

이 문제는 애너그램 그룹화에서 살펴본 예제와 유사하다. 애너그램 예제에서는 정렬된 문자열을 이용해 그룹을 식별했지만, 이 문제에서는 자릿수 합을 이용해 그룹을 식별한다. 배열을 순회하며 자릿수 합이 같은 모든 숫자를 해시 맵으로 그룹화한다. 그 다음, 해시 맵을 순회하며 요소가 2개 이상인 각 그룹에서 최대 값을 두 개 찾을 수 있다.

```cpp
class Solution {
public:
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
};
```

이 방법은 정렬 때문에 비효율적일 수 있다. 최악의 경우, 모든 숫자의 자릿수 합이 같으면 $$O(n \cdot \log{}n)$$의 시간 복잡도가 발생한다. 그룹에 있는 모든 숫자를 저장할 필요는 없다. 각 자릿수의 합에 대해 지금까지 나타난 가장 큰 숫자 두 개만 저장하면 시간 및 평균 공간 복잡도를 개선할 수 있다.

```cpp
class Solution {
public:
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
};
```

첫 번째 알고리즘은 해시 맵의 각 키에 모든 요소를 저장하기 때문에 항상 $$O(n)$$의 공간 복잡도를 가진다. 반면 개선된 알고리즘은 각 키마다 최대 두 개의 숫자만 저장하므로 평균적으로 훨씬 적은 공간을 사용한다. 더욱이 각 순회에서 추가적인 순회와 정렬이 없으므로 시간 복잡도도 $$O(n)$$이다.

---

> **예제 4**: [동일한 행 및 열 쌍](https://leetcode.com/problems/equal-row-and-column-pairs/)
>
> `n x n` 행렬 `grid`가 주어졌을 때, `R`이 행이고 `C`가 열인 쌍 `(R, C)`의 수를 반환하고, 1D 배열로 간주하면 `R`과 `C`는 동일하다.
{: .prompt-general }

동일한 쌍의 수는 어떻게 계산할까? 예를 들어, `[1, 2, 3]`처럼 보이는 행이 3개 있고, 같은 모양의 열이 2개 있다면 `세 행` 각각에 대해 쌍을 이루는 `열이 두 개` 있으므로 `총 6개의 쌍`이 있다. 해시 맵을 사용하여 각 행이 몇 번이나 나타나는지 계산하고, 두 번째 해시 맵을 사용하여 열에 대해 동일한 작업을 수행한다. 그런 다음 행 해시 맵을 반복하여 각 행에 대해 동일한 배열이 열에 몇 번 나타나는지 확인한다. 동일한 배열이 있으면 해당 횟수만큼 답에 추가한다.

문제점은 배열이 변경 가능하므로 해시 맵의 키로 사용할 수 없다는 것이다. 따라서 행과 열을 해시 가능한 형태로 변환해야 한다.

```cpp
class Solution {
public:
    int equalPairs(vector<vector<int>>& grid) {
        // C++의 map은 벡터를 키로 사용할 수 있다.
        // 그러나 map은 unordered_map 대신 O(log n)의
        // 연산을 가지지만, 여전히 $$O(n)$$보다 훨씬 빠르다.
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
};
```

---

그리드의 크기가 $$n \cdot n$$일 경우, 이 알고리즘의 시간 복잡도는 $$O(n^2)$$이다. 해시 맵을 작성하고 순회하는 작업이 이 복잡도에 의해 결정된다. 공간 복잡도 역시 $$O(n^2)$$이다.

---

## **마무리**

해시 맵을 사용한 문제를 계속해서 살펴볼 수 있지만 다른 주제도 다룰 것이다. 해시 맵을 활용하는 문제는 앞으로도 계속 나올 것이다. 해시 맵이 다양한 알고리즘에서 어떻게 활용될 수 있는지는 정말 놀랍다. 지금까지 학습한 내용을 바탕으로 몇 가지 연습 문제와 해싱 퀴즈를 풀어보는 것도 좋다.

---

## **보너스 문제**

### 존재 여부 확인

- 217. [중복 포함](https://leetcode.com/problems/contains-duplicate/)
- 1436. [목적지 도시](https://leetcode.com/problems/destination-city/)
- 1496. [경로 교차점](https://leetcode.com/problems/path-crossing/)

### 카운팅

- 1748. [고유 요소의 합계](https://leetcode.com/problems/sum-of-unique-elements/)
- 1394. [배열에서 행운의 정수 찾기](https://leetcode.com/problems/find-lucky-integer-in-an-array/)
- 1207. [고유 발생 횟수](https://leetcode.com/problems/unique-number-of-occurrences/)
- 451. [빈도별로 문자 정렬](https://leetcode.com/problems/sort-characters-by-frequency/)
- 1512. [좋은 쌍의 수](https://leetcode.com/problems/number-of-good-pairs/)
- 930. [합계가 있는 이진 부분 배열](https://leetcode.com/problems/binary-subarrays-with-sum/)
- 1695. [최대 삭제 값](https://leetcode.com/problems/maximum-erasure-value/)
- 567. [문자열의 순열](https://leetcode.com/problems/permutation-in-string/)

### 일반

- 205. [동형 문자열](https://leetcode.com/problems/isomorphic-strings/)
- 290. [단어 패턴](https://leetcode.com/problems/word-pattern/)
- 791. [사용자 지정 정렬 문자열](https://leetcode.com/problems/custom-sort-string/)
- 1657. [두 문자열이 가까운지 확인](https://leetcode.com/problems/determine-if-two-strings-are-close/)
