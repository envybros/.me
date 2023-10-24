---
title: "간격 (Interval)"
categories: [Algorithm 연구소]
tags: [Algorithm]
date: 2023-10-12 03:30
math: true
img_path: /assets/img/algorithm/
---

---

![Title](algorithm_title.png)

---

## **개요**

또 다른 흔한 패턴은 `[start, end]`과 같은 간격을 다루는 문제들이다. 보통 이런 문제들은 먼저 입력값을 정렬한 다음, 인접한 간격들을 비교하면서 반복을 돌며 해결할 수 있다. 몇 가지 예시를 살펴보자.

---

## **예제 1: 회의실 예약**

> [문제 링크](https://leetcode.com/problems/meeting-rooms/)
>
> 회의 시간 간격 배열이 주어지는데 여기서 `intervals[i] = [start, end]`는 $$i$$번째 회의가 `[start, end)` 동안 진행된다는 것을 나타낸다. 한 사람이 모든 회의에 참석할 수 있는지 확인하라.
>
> 예를 들어, `intervals = [[0, 30], [5, 10], [15, 20]]`인 경우, `false`를 반환한다. `[0, 30]` 회의에 참석하는 경우, 다른 두 회의에는 참석할 수 없다.
{: .prompt-general }

가장 간단한 방법은 모든 회의 쌍을 보고 겹치는 부분이 있는지 확인하는 것인데, 이는 $$O(n^2)$$의 시간이 걸린다. 여기서 $$n$$은 회의의 수이다. 이 문제를 개선하려면 입력값을 정렬했을 때, 충돌하는 회의들이 인접해 있을 것이라는 점을 관찰하면 된다.

회의를 시작 시간에 따라 정렬하면, 회의들이 참석해야 하는 순서대로 정렬된다. $$i$$번째 회의가 $$(i-1)$$번째 회의가 끝나기 전에 시작하면, 충돌이 발생한다. 왜냐하면 $$(i-1)$$번째 회의를 일찍 떠나 $$i$$번째 회의에 제때 도착해야 하기 때문이다.

```cpp
bool canAttendMeetings(vector<vector<int>>& intervals) {
    sort(intervals.begin(), intervals.end());
    for (int i = 1; i < intervals.size(); i++) {
        if (intervals[i][0] < intervals[i - 1][1]) {
            return false;
        }
    }
    
    return true;
}
```

정렬로 인해, 이 알고리즘의 시간 복잡도는 $$O(n \cdot \log{}n)$$이다. 이는 여전히 brute force 방식의 $$O(n^2)$$보다 훨씬 낫다. 사용되는 공간의 양은 사용하는 언어에 따라 달라진다. 예를 들어, 파이썬은 timsort를 구현하는데 최대 $$O(n)$$의 공간을 사용하지만, quicksort를 사용하는 언어는 $$O(\log{}n)$$의 공간을 사용한다.

---

## **예제 2: 간격 병합**

> [문제 링크](https://leetcode.com/problems/merge-intervals/)
>
> 간격 배열이 주어지는데 여기서 `intervals[i] = [start, end]`로, 모든 겹치는 간격을 병합하고 입력에서 모든 간격을 커버하는 비겹치는 간격 배열을 반환해보자.
>
> 예를 들어, `intervals = [[1, 3], [2, 6], [8, 10], [15, 18]]`인 경우, `[[1, 6], [8, 10], [15, 18]]`을 반환한다. 처음 두 간격은 `[1, 6]`을 형성하기 위해 병합되고, 그 다음 두 간격은 겹치지 않는다.
{: .prompt-general }

이전 문제에서 우리는 간격을 시작 시간별로 정렬했고, 그런 다음 인접한 간격들을 비교하여 겹치는 부분이 있는지 확인했다. 여기서도 같은 과정을 사용할 수 있다. 겹치는 두 간격이 있다고 가정하자 - `[a, b]`와 `[c, d]`. 병합된 간격은 `[min(a, c), max(b, d)]`가 되어야 한다.

입력을 시작 시간별로 정렬하고 정답 배열 `ans`를 한 간격씩 구축하자. 어떤 주어진 간격에서, 만약 `start`가 이전 간격의 종료 시간보다 작거나 같다면, 겹치는 부분이 있고 우리는 간격을 병합해야 한다. 시작 시간별로 정렬하기 때문에, 이전 간격의 `start`는 항상 더 작으므로 이전 단락의 `min(a, c)` 부분은 무시할 수 있다. 우리는 단지 새로운 끝을 `max(b, d)`로 수정해야 한다.

```cpp
class Solution {
public:
    vector<vector<int>> merge(vector<vector<int>>& intervals) {
        sort(intervals.begin(), intervals.end());
        vector<vector<int>> ans;
        
        for (vector<int>& interval: intervals) {
            int start = interval[0], end = interval[1];
            if (!ans.empty() && start <= ans[ans.size() - 1][1]) {
                ans[ans.size() - 1][1] = max(ans[ans.size() - 1][1], end);
            } else {
                ans.push_back(interval);
            }
        }
        
        return ans;
    }
};
```

이전 예시와 마찬가지로, 정렬하는 데 $$O(n \cdot \log{}n)$$의 비용이 든다. 여기서 $$n$$은 간격의 수이다. 만약 추가 공간을 정답으로 계산하지 않는다면, 이 알고리즘은 이전과 동일한 공간을 사용하는데, 이는 정렬 구현에 따라 달라진다.

---

출처: [Leetcode](https://leetcode.com/explore/interview/card/leetcodes-interview-crash-course-data-structures-and-algorithms/714/bonus/4650/)

<!--

{: .prompt-general }

-->