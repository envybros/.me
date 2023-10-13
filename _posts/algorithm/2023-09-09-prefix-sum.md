---
title: "누적 합 (Prefix sum)"
categories: [Algorithm 연구소]
tags: [Algorithm]
date: 2023-09-09 01:30
math: true
img_path: /assets/img/algorithm/
---

---

![Title](algorithm_title.png)

---

## **개요**

누적 합은 배열에 적용할 수 있는 기법이다. 기본 아이디어는 `prefix[i]`를 배열 `nums`의 인덱스 `0`번 부터 인덱스 `i`번까지의 합으로 설정하는 것이다. 예를 들면, `nums = [5, 2, 1, 6, 3, 8]`이라면 `prefix = [5, 7, 8, 14, 17, 25]`가 된다.

> 부분 배열이 인덱스 `0`에서 시작하면 그 배열을 "prefix"로 볼 수 있다. 누적 합은 연속된 숫자들의 합을 빠르게 계산할 수 있게 해준다.
{: .prompt-general }

누적 합을 사용하면 모든 부분 배열의 합을 $$O(1)$$로 구할 수 있다. `i`에서 `j`까지의 부분 배열의 합(포함)을 구하려면, `prefix[j] - prefix[i - 1]`으로 계산하면 된다. 만약 `i = 0`일 때 인덱스를 벗어나는 것을 처리하고 싶지 않다면, `prefix[j] - prefix[i] + nums[i]`로 계산할 수 있다.

이러한 계산 방법은 `prefix[i - 1]`이 인덱스 `i` **이전**의 모든 요소의 합이기 때문에 가능하다. 인덱스 `j`까지의 모든 요소의 합에서 이 값을 빼면, 인덱스 `i`에서 시작하여 인덱스 `j`에서 끝나는 모든 요소의 합이 남게 된다. 이 값이 바로 우리가 원하는 부분 배열의 합이다.

![Prefix](Prefixsum.png)

> 위 이미지에서 파란색으로 표시된 부분 배열의 합을 구하려 한다.
>
> 부분 배열의 끝(녹색 선)까지의 합에서 그 이전의 합(빨간색 선)을 빼면 해당 부분 배열의 합을 얻을 수 있다.
>
> 누적 합을 이용하면, 녹색 선 값 `25`에서 빨간색 선 값 `11`을 빼서 부분 배열의 합 `14`를 상수 시간 안에 구할 수 있다.
{: .prompt-general }

누적 합을 만드는 것은 매우 간단하다. 다음은 누적 합을 만드는 의사 코드이다:

```cpp
Given an array nums,

prefix = [nums[0]]
for (int i = 1; i < nums.length; i++)
    prefix.append(nums[i] + prefix[prefix.length - 1])
```

처음에는 첫 번째 요소로 시작한다. 그런 다음 인덱스 `1`부터 `i`로 반복한다. 특정 시점에서 `prefix`의 마지막 요소는 인덱스 `i` 이전의 모든 요소의 합계를 나타낸다. 따라서 이 값에 현재 값을 더해서 `prefix`에 추가하고, 다음 요소로 넘어간다.

누적 합은 부분 배열의 합을 구할 때 유용하다. 생성하는 데 $$O(n)$$의 시간이 걸리지만, 이후의 모든 부분 배열의 합은 $$O(1)$$의 시간에 구할 수 있기 때문에, 전체적인 알고리즘의 시간 복잡도를 크게 줄일 수 있다. 이제 몇 가지 예를 살펴볼 것이다.

> 누적 합을 만드는 것은 **전처리(pre-processing)**의 한 형태다. 전처리는 데이터를 미리 계산해두어 저장하는 방식으로, 다양한 문제에서 유용하게 사용된다. 전처리 과정에 시간이 조금 걸리더라도, 이를 통해 메인 로직의 실행 시간을 크게 줄일 수 있다.
{: .prompt-general }

---

> **예제 1**: 정수 배열 `nums`와 `queries[i] = [x, y]` 형태의 배열 `queries`, 그리고 정수 `limit`이 주어질 때, 각 쿼리의 결과로 이루어진 boolean 배열을 반환한다. `x`부터 `y`까지의 부분 배열의 합이 `limit`보다 작다면 해당 쿼리의 결과는 `true`, 그렇지 않다면 `false`이다.
>
> 예를 들어, `nums = [1, 6, 3, 2, 7, 2]`, `queries = [[0, 3], [2, 5], [2, 4]]`, `limit = 13`이 주어지면, 결과는 `[true, false, true]`이다. 각 쿼리에 대한 부분 배열의 합은 `[12, 14, 12]`이다.
{: .prompt-general }

누적 합을 구한 후에, 위에서 설명한 방법을 사용하여 각 쿼리에 대한 결과를 $$O(1)$$의 시간 안에 구할 수 있다.

```cpp
vector<bool> answerQueries(vector<int>& nums, vector<vector<int>>& queries, int limit) {
    vector<int> prefix = {nums[0]};
    for (int i = 1; i < nums.size(); i++) {
        prefix.push_back(prefix.back() + nums[i]);
    }
    
    vector<bool> ans;
    for (vector<int>& query: queries) {
        int x = query[0], y = query[1];
        int curr = prefix[y] - prefix[x] + nums[x];
        ans.push_back(curr < limit);
    }
    
    return ans;
}
```

누적 합이 없으면 각 쿼리에 대한 응답은 최악의 경우 $$O(n)$$이 되며, 여기서 $$n$$은 `nums`의 길이이다. `m = queries.length`인 경우, 시간 복잡도는 $$O(n * m)$$이 된다. 누적 합을 사용하면 빌드하는 데 $$O(n)$$의 비용이 들지만 각 쿼리에 응답하는 데는 $$O(1)$$이 소요된다. 이렇게 하면 시간 복잡도가 $$O(n + m)$$으로 훨씬 더 좋아진다. 누적 합을 구축하기 위해 $$O(n)$$ 공간을 사용한다.

---

> **예제 2**: [배열을 분할하는 방법 수](https://leetcode.com/problems/number-of-ways-to-split-array/)
>
> 정수 배열 `nums`가 주어졌을 때, 첫 번째 부분의 합이 두 번째 부분의 합보다 크거나 같도록 배열을 두 부분으로 분할하는 방법의 개수를 구한다. 두 번째 부분에는 적어도 하나의 숫자가 있어야 한다.
{: .prompt-general }

무차별 대입 방식은 각 인덱스 `i`를 `0`에서 `nums.length - 1`까지 반복하는 것이다. 각 인덱스에 대해 `0`에서 `i`까지 반복하여 왼쪽 부분의 합을 구한 다음 `i + 1`에서 배열의 끝까지 반복하여 오른쪽 부분의 합을 구한다. 이 알고리즘의 시간 복잡도는 $$O(n^2)$$이다.

누적 합을 먼저 만든 다음 각 인덱스에 대해 반복하면 왼쪽과 오른쪽 부분의 합을 $$O(1)$$로 계산할 수 있으므로 시간 복잡도을 $$O(n)$$으로 개선할 수 있다.

```cpp
class Solution {
public:
    int waysToSplitArray(vector<int>& nums) {
        int n = nums.size();
        
        vector<long> prefix = {nums[0]};
        for (int i = 1; i < n; i++) {
            prefix.push_back(nums[i] + prefix.back());
        }
        
        int ans = 0;
        for (int i = 0; i < n - 1; i++) {
            long leftSection = prefix[i];
            long rightSection = prefix.back() - prefix[i];
            if (leftSection >= rightSection) {
                ans++;
            }
        }
        
        return ans;
    }
};
```

---

## **배열이 필요할까?**

이 문제에서 `prefix`에 접근해야 하는 순서는 순차적이다. `왼쪽 부분`을 찾으려면 반복할 때마다 `i`가 `1`씩 증가함에 따라 `prefix[i]`를 조회하면 된다.

따라서 `왼쪽 부분`을 계산하기 위해 실제로 배열이 필요하지 않다. `leftSection = 0`으로 초기화한 다음 각 반복마다 현재 요소를 추가하여 즉석에서 계산할 수 있다.

`오른쪽 부분`은 어떨까? 정의에 따라 `오른쪽 부분`에는 `왼쪽 부분`에 없는 배열의 모든 숫자가 포함된다. 따라서 전체 입력의 합계를 총합으로 미리 계산한 다음, `오른쪽 부분`을 `total - leftSection`으로 계산할 수 있다.

`왼쪽 부분`의 각 값은 누적 합을 나타내므로 여전히 누적 합의 개념을 사용하고 있다. 배열 대신 정수를 사용하여 이 기능을 단순화했다.

```cpp
class Solution {
public:
    int waysToSplitArray(vector<int>& nums) {
        int ans = 0;
        long leftSection = 0;
        long total = 0;
        
        for (int num: nums) {
            total += num;
        }
        
        for (int i = 0; i < nums.size() - 1; i++) {
            leftSection += nums[i];
            long rightSection = total - leftSection;
            if (leftSection >= rightSection) {
                ans++;
            }
        }
        
        return ans;
    }
};
```

공간 복잡도를 $$O(1)$$로 개선하여 성능이 크게 개선되었다.

---

## **마무리**

이것이 배열과 문자열에 대해 살펴볼 마지막 주요 패턴이다.

---

출처: [Leetcode](https://leetcode.com/explore/interview/card/leetcodes-interview-crash-course-data-structures-and-algorithms/703/arraystrings/4503/)
