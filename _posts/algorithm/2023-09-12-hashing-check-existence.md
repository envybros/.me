---
title: "[해싱] 존재 여부 확인"
categories: [Algorithm 연구소]
tags: [Algorithm]
date: 2023-09-12 01:10
math: true
img_path: /assets/img/algorithm/
---

---

![Title](algorithm_title.png)

---

## **개요**

해시 테이블이나 집합에서 가장 흔한 사용 사례 중 하나는 요소의 존재 여부를 $$O(1)$$의 시간 복잡도로 확인하는 것이다. 이러한 작업을 배열에서 수행하면 $$O(n)$$의 시간이 필요하다. 따라서 해시 테이블이나 집합을 사용하면 알고리즘의 시간 복잡도를 일반적으로 $$O(n^2)$$에서 $$O(n)$$으로 줄일 수 있다. 이에 대한 예제 문제를 살펴보자.

---

> **예제 1**: [Two Sum](https://leetcode.com/problems/two-sum/)
>
> 주어진 배열 `nums`와 `target`을 가지고, 두 숫자의 합이 `target`이 되는 숫자들의 인덱스를 반환한다. 동일한 인덱스는 두 번 사용할 수 없다.
{: .prompt-general }

무차별 대입 방식의 해결책은 모든 인덱스 조합을 확인하기 위해 중첩된 for 루프를 사용하며, 그 합이 `target`과 일치하는지 확인한다. 이러한 접근 방식은 시간 복잡도가 $$O(n^2)$$이 된다. 첫 번째 for 루프는 `num`에 초점을 맞추고, 두 번째 for 루프는 배열에서 `target - num`을 찾는다. 배열을 사용하면 `target - num`을 찾는 데 $$O(n)$$의 시간이 걸리지만, 해시 테이블을 사용하면 $$O(1)$$의 시간이 걸린다.

배열을 순회하면서 각 값을 해당 인덱스와 연결하여 해시 테이블을 만들 수 있다. 각 인덱스 `i`에서 `num = nums[i]`일 때, 해시 테이블에서 `target - num`을 확인할 수 있다. 키-값 쌍을 추가하고 `target - num`을 확인하는 모든 작업은 $$O(1)$$이므로 전체 시간 복잡도는 $$O(n)$$으로 줄어든다.

```cpp
class Solution {
public:
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
};
```

> 만약 문제가 값의 쌍이 존재하는지만 확인하거나 값을 반환하도록 요구한다면, 단순히 집합을 사용할 수 있다. 하지만 숫자의 인덱스를 반환하도록 요구하기 때문에, 어떤 인덱스에 해당 숫자가 위치하는지를 "기억하기" 위해 해시 테이블을 사용한다.
{: .prompt-general }

해시 테이블의 연산은 $$O(1)$$의 시간 복잡도를 가지므로, 전체 알고리즘의 시간 복잡도는 $$O(n)$$이다. 또한, 이 해결책은 입력 크기에 따라 선형적으로 증가하는 키의 수를 저장하기 위해 해시 테이블을 사용하므로 $$O(n)$$의 공간을 사용한다.

---

> **예제 2**: [첫 글자가 두 번 나타나는 경우](https://leetcode.com/problems/first-letter-to-appear-twice/)
>
> 문자열 `s`가 주어졌을 때, 두 번 나타나는 첫 번째 문자를 반환한다. 주어진 입력에는 반드시 중복 문자가 포함되어 있다.
{: .prompt-general }

무차별 대입 방식의 해결책은 문자열을 순회하면서 각 문자 `c`에 대해 이전 문자들을 다시 검사하여 중복을 찾는 것이다.

```cpp
class Solution {
public:
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
};
```

중첩된 for문으로 인해 시간 복잡도는 $$O(n^2)$$이다. 두 번째 for문은 문자 `c`의 중복 여부를 확인하는데, 이를 집합을 사용해 $$O(1)$$의 시간 복잡도로 할 수 있다.

```cpp
class Solution {
public:
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
};
```

각 for문의 반복은 상수 시간에 실행되므로 전체 시간 복잡도는 $$O(n)$$이다.

공간 복잡도에 대한 토론은 더 흥미롭다. 많은 사람들은 입력 문자열이 알파벳 문자만 포함하므로 최대 26개의 문자만 저장할 수 있다고 생각하여 공간 복잡도를 $$O(1)$$이라고 생각할 수 있다. 이는 문자열 문제에서 자주 나오는 생각이며 기술적으로 맞다. 그러나 면접이나 다른 상황에서는 더 일반적인 상황을 고려하여 공간 복잡도를 $$O(m)$$이라고 설명할 수 있으며, 여기서 $$m$$은 입력에서 가능한 문자의 수를 나타낸다. 이는 더 일반적인 경우를 고려한 것이며, 이것도 역시 정확하다.

---

> **예제 3**: 정수 배열 `nums`가 주어졌을 때, 다음 조건을 만족하는 숫자 `x`가 `nums` 안에 모두 있는지 찾아보자: `x + 1`이 `nums`에 포함되지 않고, `x - 1`이 `nums`에 포함되지 않는 경우.
{: .prompt-general }

`nums`를 순회하며 `x + 1`이나 `x - 1`이 배열에 있는지 확인하면 된다. 이러한 확인 작업을 $$O(1)$$의 시간 내에 수행하려면 `nums`를 미리 집합으로 변환하면 된다.

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

검사하는 데 걸리는 시간은 $$O(1)$$이므로 전체 시간 복잡도는 $$O(n)$$이다. 집합은 $$O(n)$$의 공간을 차지한다.

---

알고리즘이 `만약 ... 안에 ... 있으면' (if ... in ...)` 형태의 연산을 수행할 때마다 해시 맵이나 집합을 사용하여 요소를 저장하고, 이 연산을 $$O(1)$$의 시간 내에 실행하는 것을 고려해야 한다. 이 내용을 참고하여 다른 연습 문제를 풀어보자.

---

출처: [Leetcode](https://leetcode.com/explore/interview/card/leetcodes-interview-crash-course-data-structures-and-algorithms/705/hashing/4511/)
