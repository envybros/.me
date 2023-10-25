---
title: "[DSA] 구간 합"
categories: [Algorithm 연구소]
tags: [Algorithm]
date: 2022-08-06 01:30
math: true
img_path: /assets/img/algorithm/
---

---

![Title](algorithm_title.png)

---

## **개요**

구간 합은 숫자 배열에 사용할 수 있는 기법이다. 이 방법은 'prefix'라는 새 배열을 만드는 것으로, prefix[i]는 인덱스 i까지(해당 인덱스 포함)의 모든 요소의 합이 되도록 구성한다. 예를 들어, nums = [5, 2, 1, 6, 3, 8] 경우, prefix = [5, 7, 8, 14, 17, 25]를 얻을 수 있게 된다.

> 부분 배열이 인덱스 0에서 시작하는 경우, 이를 배열의 "구간"으로 본다. 구간 합은 이러한 구간들의 합을 의미한다.
{: .prompt-tip }

이 구간 합 기법을 사용하면 어떤 부분 배열의 합도 O(1) 시간 내에 구할 수 있다. i부터 j(포함)까지의 부분 배열의 합이 필요한 경우, 결과는 prefix[j] - prefix[i - 1]이 되며, i = 0인 상황에서 범위를 초과하는 문제를 처리하고 싶지 않다면, prefix[j] - prefix[i] + nums[i]를 사용하면 된다.

이러한 계산이 가능한 이유는 prefix[i - 1]이 인덱스 i 이전의 모든 원소들의 합이기 때문이다. 따라서 인덱스 j까지의 모든 원소의 합에서 이 값을 빼면, 인덱스 i에서 j까지의 원소들의 합, 즉 우리가 찾는 값이 남게 된다. 이러한 방식으로, 필요한 부분 배열의 합을 효과적으로 찾을 수 있다.

![Prefix](Prefixsum.png)

> 위 이미지에서, 우리는 파란색으로 강조된 부분 배열의 합을 찾고자 한다.
>
> 부분 배열 끝까지의 모든 요소(초록색 선)를 가져와 그것 이전의 모든 요소(빨간색 선)를 빼면 원하는 부분 배열을 얻을 수 있게 된다. 구간 합을 사용하면, 초록색 선의 합인 25와 빨간색 선의 합인 11을 상수 시간 안에 찾아, 그 차이를 이용하여 부분 배열의 합이 14임을 알 수 있다.
{: .prompt-tip }

구간 합을 만드는 것은 매우 간단하다. 다음은 의사 코드이다:

```text
nums라는 배열이 주어졌을 때,

prefix = [nums[0]]
for (int i = 1; i < nums.length; i++)
    prefix.append(nums[i] + prefix[prefix.length - 1])
```

처음에는 첫 번째 요소로 시작한다. 그리고는 인덱스 1부터 i를 이용하여 반복하면서 진행한다. 어떤 시점에서건, prefix의 마지막 요소는 인덱스 i까지의 모든 요소들의 합을 나타낸다. 따라서 우리는 그 값을 현재 값에 더하여 prefix의 마지막에 추가하고 다음 요소로 넘어갈 수 있다.

구간 합은 부분 배열의 합과 관련된 문제를 해결함에 있어 매우 유용한 도구이다. 구축하는 데 드는 비용은 $O(n)$이지만, 이후의 모든 부분 배열 조회를 $O(1)$에 수행할 수 있게 해주므로, 보통 알고리즘의 시간 복잡도를 $O(n)$만큼 향상시킬 수 있다. 이 때 $n$은 배열의 길이를 나타낸다. 다양한 예시를 통해 더 자세히 이해해보자.

> 구간 합 구축은 **전처리**의 일종이다. 전처리는 알고리즘의 핵심 로직을 실행하기 전에 데이터 구조에 미리 계산된 데이터를 저장함으로써, 다양한 문제 상황에서 유용하게 사용된다. 전처리 과정에 시간이 소요되기는 하지만, 이는 알고리즘의 주요 단계에서 크게 시간을 절약할 수 있는 투자라고 볼 수 있다.
{: .prompt-tip }

---

## **예제 1: 부분 배열 합 쿼리 처리하기**

> 정수 배열 'nums'와 'queries' 배열이 주어지며, 이때 queries[i] = [x, y] 형태이다. 또한 정수 'limit'이 주어진다. 이에 대한 답으로, 각 쿼리의 결과를 나타내는 부울 배열을 반환해야 한다. 만약 x부터 y까지의 부분 배열의 합이 'limit'보다 작다면 해당 쿼리의 결과는 'true'이며, 그렇지 않은 경우 'false'이다.
>
> 예를 들어, nums = [1, 6, 3, 2, 7, 2], queries = [[0, 3], [2, 5], [2, 4]], 그리고 limit = 13이라면, 반환되는 답은 [true, false, true]이다. 각 쿼리에 대한 부분 배열의 합은 [12, 14, 12]이다.
{: .prompt-tip }

이제 구간 합을 만들고, 위에서 설명한 방법을 이용하여 각 쿼리에 대한 답을 $O(1)$의 복잡도로 찾아보자.

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

```rs
fn answer_queries(nums: Vec<i32>, queries: Vec<Vec<i32>>, limit: i32) -> Vec<bool> {
    let mut prefix = vec![nums[0]];
    for i in 1..nums.len() {
        let last = *prefix.last().unwrap();
        prefix.push(last + nums[i]);
    }

    let mut ans = Vec::new();
    for query in queries {
        let (x, y) = (query[0] as usize, query[1] as usize);
        let curr = prefix[y] - prefix[x] + nums[x];
        ans.push(curr < limit);
    }

    ans
}
```

구간 합 없이 각 쿼리를 해결하는 것은 최악의 경우 $O(n)$의 시간 복잡도를 가지며, 여기서 $n$은 'nums'의 길이를 나타낸다. m이 queries의 길이라고 할 때, 시간 복잡도는 $O(n*m)$이 된다. 반면, 구간 합을 이용하면, 이를 생성하는 데 $O(n)$이 소요되지만 각 쿼리에 대한 답은 $O(1)$로 줄어든다. 이 방법은 $O(n+m)$의 더 나은 시간 복잡도를 제공한다. 구간 합 배열을 생성하기 위한 공간 복잡도는 $O(n)$이 필요하다.

이러한 접근 방식을 통해, 주어진 'limit' 조건 하에서 'queries'에 명시된 부분 배열의 합을 효율적으로 계산할 수 있다. 구간 합은 초기 배열 설정에 시간이 소요되긴 하지만, 이후 쿼리 처리 시간을 대폭 줄여준다는 장점이 있다.

---

## **예제 2: 배열 분할 방법의 수**

> [문제 링크](https://leetcode.com/problems/number-of-ways-to-split-array/)
>
> 정수 배열 nums가 주어졌을 때, 첫 번째 부분의 합이 두 번째 부분의 합보다 크거나 같게 배열을 두 부분으로 나누는 방법의 수를 찾아보자. 두 번째 부분은 최소 한 개의 숫자를 포함해야 한다.
{: .prompt-tip }

이 문제를 해결하는 직접적인 방법은 각 인덱스 i에 대해 0부터 nums.length - 1까지 반복하는 것이다. 각 인덱스에서 0부터 i까지 반복하여 왼쪽 부분의 합을 구하고, i + 1부터 배열의 끝까지 반복하여 오른쪽 부분의 합을 구한다. 이 알고리즘의 시간 복잡도는 $O(n^2)$이다.

하지만, 먼저 구간 합을 계산하면 각 인덱스에 대한 반복 과정 중에 왼쪽 및 오른쪽 부분의 합을 $O(1)$에 알 수 있으므로 시간 복잡도를 $O(n)$으로 줄일 수 있다.

### 예제 2 상세 설명

배열을 두 부분으로 나누게 되면 두 부분 배열이 생성된다. 이것들의 합을 찾아 비교하는 과정이 필요하다. 배열을 나누는 데는 $n-1$가지 방법이 있으며(오른쪽 부분은 비어 있지 않아야 함), 이 각각의 분할에 대해 두 부분 배열 사이를 반복하며 그 합을 찾는 데 $O(n)$이 걸린다.

그러나 분할을 시도하기 전에 한 번 $O(n)$을 소모하여 구간 합을 만들 수 있다. 이렇게 하면 각각의 $n-1$ 분할을 $O(1)$ 시간 안에 수행할 수 있다는 장점이 있다. 알려진 바와 같이, 구간 합을 활용하면 어떤 부분 배열의 합도 $O(1)$에 구할 수 있다.

예를 들어, 인덱스 i에서 배열을 나눈다고 가정하자. 왼쪽 부분은 인덱스 i까지의 모든 요소를 포함하므로 prefix[i]의 합을 갖는다. 오른쪽 부분은 인덱스 i + 1부터 시작하여 최종 인덱스 n - 1에서 끝나므로, prefix[n - 1]에서 prefix[i]를 뺀 값이 그 합이 된다.

```cpp
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
```

```rs
fn ways_to_split_array(nums: Vec<i32>) -> i32 {
    let n = nums.len();
    let mut prefix = Vec::new();
    prefix.push(nums[0] as i64); // long

    for i in 1..n {
        prefix.push(nums[i] as i64 + prefix[i - 1]);
    }

    let mut ans = 0;
    for i in 0..n - 1 {
        let left_section = prefix[i];
        let right_section = prefix[n - 1] - prefix[i];
        if left_section >= right_section {
            ans += 1;
        }
    }

    ans
}
```

---

## **배열이 정말 필요할까?**

이 문제에서는 구간에 접근하는 순서가 점진적으로 증가한다. leftSection을 찾기 위해서는 각 반복마다 1씩 증가하는 i에 대해 prefix[i]를 수행한다.

이러한 점을 감안할 때, leftSection을 계산하는 데 배열은 실제로 필요하지 않다. leftSection을 0으로 초기화하고, 각 반복에서 현재 원소를 더하는 방식으로 즉석에서 계산할 수 있다.

그렇다면 rightSection은 어떨까? 정의에 따르면 오른쪽 섹션은 왼쪽 섹션에 없는 배열의 모든 숫자를 포함한다. 그러므로, 우리는 전체 입력의 합을 total로 미리 계산할 수 있으며, rightSection은 total - leftSection으로 계산할 수 있다.

구간 합의 개념을 여전히 사용하고 있지만, leftSection의 각 값은 구간의 합을 나타낸다. 배열이 아닌 정수를 사용하여 기능을 복제하기만 하면 된다.

```cpp
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
```

```rs
fn ways_to_split_array(nums: Vec<i32>) -> i32 {
    let mut ans = 0;
    let mut left_section = 0;
    let total: i32 = nums.iter().sum();

    for i in 0..nums.len() - 1 {
        left_section += nums[i];
        let right_section = total - left_section;
        if left_section >= right_section {
            ans += 1;
        }
    }

    ans
}
```

이렇게 하여 공간 복잡도를 $O(1)$로 개선하였고, 이는 성능이 크게 개선된 것이다.

---

## **마무리**

배열과 문자열에 대한 주요 패턴의 마지막 챕터를 살펴보았다. 다음 문서에서는 몇 가지 더 일반적인 트릭과 패턴을 다룰 예정이다.

## **보너스 문제**

- [1480. Running Sum of 1d Array](https://leetcode.com/problems/running-sum-of-1d-array/)
- [1413. Minimum Value to Get Positive Step by Step Sum](https://leetcode.com/problems/minimum-value-to-get-positive-step-by-step-sum/)
- [2090. K Radius Subarray Averages](https://leetcode.com/problems/k-radius-subarray-averages/)
- [1732. Find the Highest Altitude](https://leetcode.com/problems/find-the-highest-altitude/)
- [724. Find Pivot Index](https://leetcode.com/problems/find-pivot-index/)
- [303. Range Sum Query - Immutable](https://leetcode.com/problems/range-sum-query-immutable/)

---

> 출처: [Leetcode](https://leetcode.com/explore/interview/card/leetcodes-interview-crash-course-data-structures-and-algorithms/703/arraystrings/4503/)
{: .prompt-info }
