---
title: "힙 예제"
categories: [Algorithm 연구소]
tags: [Algorithm]
date: 2023-09-26 01:30
math: true
img_path: /assets/img/algorithm/
---

---

![Title](algorithm_title.png)

---

## **개요**

힙은 반복적으로 최대 또는 최소 요소를 찾아야 할 때 놀라운 도구다. 몇 가지 예제 문제를 살펴보자.

## **예제 1: 마지막 돌의 무게**

> [문제 링크](https://leetcode.com/problems/last-stone-weight/)
>
> `stones[i]`가 $$i^{th}$$ 돌의 무게인 정수 배열 `stones`가 주어진다. 각 턴에서, 가장 무거운 두 개의 돌을 선택하고 함께 부딪힌다. 가장 무거운 두 돌의 무게가 `x`와 `y`이고 `x <= y`라고 가정해보자. `x == y`이면 두 돌 모두 파괴된다. `x != y`이면 `x`가 파괴되고 `y`는 `x`의 무게를 잃는다. 남아있는 마지막 돌의 무게를 반환하거나, 돌이 하나도 없으면 `0`을 반환한다.
{: .prompt-general }

이 문제에서, 우리는 반복적으로 2개의 최대 요소를 찾아야 한다. `stones`를 최대 힙으로 변환해서 두 개의 최대 요소를 $$O(\log{}n)$$에 팝하고, 스매시를 수행한 다음 두 돌이 모두 파괴되지 않았다면 $$O(\log{}n)$$에 다시 힙에 추가할 수 있다. 돌이 하나 또는 없을 때까지 이 과정을 계속할 수 있다.

### [예제 1] 상세 설명

이 문제를 해결하기 위해서는 과정을 그대로 시뮬레이션하면 된다. 문제는 두 개의 가장 무거운 돌을 반복적으로 찾는 것이 비용이 많이 들 수 있다는 것이다. 입력값을 내림차순으로 정렬하고 순서대로 요소를 거치는 것만으로는 충분하지 않다. 왜냐하면 대부분 스매시 결과로 새로운 돌이 생겨서 입력값에 다시 넣어야 하기 때문이다. 힙을 사용하면 로그 시간에 두 개의 최대 요소를 제거할 수 있다. 스매시를 수행한 후 남은 돌이 있다면 로그 시간에 다시 추가할 수 있다. 로그 시간은 선형 시간보다 **훨씬** 빠르기 때문에, 이는 일반 배열을 사용하는 것보다 엄청난 개선이다. 그래서 모든 돌을 최대 힙에 넣는다. 그런 다음 돌이 하나 또는 없을 때까지 과정을 시뮬레이션한다. 2개의 최대 요소를 팝하고 문제 설명의 규칙을 적용한다. 내부에서 무슨 일이 일어나는지에 집중하지 말고, 힙이 당신을 위해 할 수 있는 일과 작업을 사용하는 방법을 기억하자.

```cpp
class Solution {
public:
    int lastStoneWeight(vector<int>& stones) {
        priority_queue<int> heap(stones.begin(), stones.end());
        
        while (heap.size() > 1) {
            int first = heap.top();
            heap.pop();
            int second = heap.top();
            heap.pop();
            if (first != second) {
                heap.push(first - second);
            }
        }
        
        return heap.empty() ? 0: heap.top();
        
    }
};
```

> 파이썬의 [힙 구현](https://docs.python.org/3/library/heapq.html)은 최소 힙만을 구현한다. 최대 힙을 시뮬레이션하기 위해 힙에 넣는 모든 값을 음수로 만들 수 있다.
{: .prompt-general }

각 스매시마다 적어도 하나의 돌이 파괴되므로 최대 `n`번의 반복을 할 수 있다. 각 반복에서, 처음에 `n`의 길이를 가진 힙에서 팝과 푸시를 수행한다. 이것은 우리에게 시간 복잡도 $$O(n \cdot \log{}n)$$을 제공한다. 힙은 $$O(n)$$ 공간을 사용한다. 파이썬에서는 입력을 재사용하므로, 보통은 하지 않을 것이지만 공간 복잡도에 대해 계산해야 한다는 점을 주의해야 한다.

---

## **예제 2: 배열 합의 절반으로 줄이기 위한 최소 작업 수**

> [문제 링크](https://leetcode.com/problems/minimum-operations-to-halve-array-sum/)
>
> 양의 정수의 `nums` 배열이 주어진다. 한 번의 작업에서 `nums`에서 아무 숫자나 선택하고 그 숫자를 정확히 절반으로 줄일 수 있다. `nums`의 합을 적어도 절반으로 줄이기 위한 최소 작업 수를 반환하라.
{: .prompt-general }

줄일 숫자를 선택하는 가장 좋은 방법은 무엇일까? 단계를 최소화하고 싶기 때문에, 각 단계에서 `nums`를 최대한 줄이고 싶다. 이는 주어진 순간에 가장 큰 요소를 선택해야 함을 의미한다. 주어진 시간에 가장 큰 요소를 추적하기 위해 입력을 최대 힙으로 변환하자. 각 단계에서 최대 `x`를 팝하고, 합에서 `x / 2`를 제거한 다음 `x / 2`를 힙에 다시 푸시한다.

### [예제 2] 상세 설명

이것은 힙을 사용할 때의 또 다른 좋은 예이다 - 우리는 반복적으로 최대 요소를 찾아야 한다. 이전 예제와 마찬가지로, 입력을 내림차순으로 정렬하고 순서대로 요소를 거치는 것으로는 충분하지 않다. 왜냐하면 요소가 절반으로 줄어든 후에 다시 추가되기 때문이다. 먼저, 입력을 힙으로 변환한다. 그런 다음 `target`을 요소의 합을 2로 나눈 것으로 정의한다 - 이것은 우리가 달성해야 할 감소량이다. 이제 `target > 0`인 동안에, 합을 줄여야 한다. 힙에서 최대 요소 x를 제거한다(빠르고 쉽다). `x / 2`로 줄이기 위해 target에서 `x / 2`를 빼고, 그런 다음 `x / 2`를 힙에 다시 넣는다. 힙은 우리에게 요소를 다시 추가하면서도 로그 시간에 항상 최대 요소를 제공할 것이다.

```cpp
class Solution {
public:
    int halveArray(vector<int>& nums) {
        priority_queue<double> heap(nums.begin(), nums.end());
        double target = 0;
        for (double num: nums) {
            target += num;
        }
        
        target /= 2;
        int ans = 0;
        while (target > 0) {
            ans++;
            double x = heap.top(); heap.pop();
            target -= (x / 2);
            heap.push(x / 2);
        }
        
        return ans;
    }
};
```

> 이전 두 예에서 볼 수 있듯이, 힙은 반복적으로 최대 또는 최소 요소를 찾아야 할 때 놀라운 자료 구조다. 로그 시간 내에 삽입과 제거를 처리하면서도 최대/최소 속성을 유지할 수 있다.
{: .prompt-general }

루프의 각 반복은 힙 작업으로 인해 $$O(\log{}n)$$ 시간이 걸린다. 필요한 작업 수는 `n`과 선형 관계다. 거대한 숫자를 가지고 있다면, 그 숫자는 여러 번 반으로 줄어들어야 할 것이라고 생각할 수 있다. 사실이지만, 그것에 대한 각 작업도 합계를 큰 폭으로 줄일 것이다. 이것은 $$O(n \cdot \log{}n)$$의 시간 복잡도를 우리에게 제공한다.

> 작업 수가 `n`으로 제한되는 이유에 대한 더 명확한 논증 - 각 숫자에 대해 작업을 한 번씩 수행할 수도 있다.
{: .prompt-general }

---

## **투 힙 (Two heaps)**

여러 힙을 사용하는 것은 드물고, 그것을 요구하는 문제들은 대체로 더 어려운 편이다. 문제가 중앙값 찾기와 관련이 있다면, 이것을 생각하는 것이 좋다. 우리가 곧 살펴볼 예제는 중앙값과 관련이 있고, [Sliding Window Median](https://leetcode.com/problems/sliding-window-median/)도 두 힙으로 해결할 수 있지만, 매우 어렵기 때문에 연습 문제로 포함하지는 않을 것이다(그래도 원한다면 나중에 시도할 수 있다).

이것은 더 어려운 희귀 개념과 같다. 이해하는 데 어려움을 겪고 있다면 낙담하지 않아도 좋다.

## **예제 3: 데이터 스트림에서 중앙값 찾기**

> [문제 링크](https://leetcode.com/problems/find-median-from-data-stream/)
>
> 중앙값은 정렬된 정수 목록의 중간 값이다. 목록의 크기가 짝수인 경우, 중앙값은 두 중간 값의 평균이다. MedianFinder 클래스를 구현하라:
>
> `MedianFinder()`는 `MedianFinder` 객체를 초기화한다.
>
> `void addNum(int num)`은 정수 `num`을 자료 구조에 추가한다.
>
> `double findMedian()`은 지금까지의 모든 요소의 중앙값을 반환한다.
{: .prompt-general }

문제는 지속적으로 추가되는 데이터 세트에서 중간 요소를 찾는 것으로 귀결된다. 최소 또는 최대 요소 대신 중간 요소를 찾기 위해 힙을 어떻게 활용할 수 있을까?

데이터의 더 큰 절반만을 저장하는 최소 힙을 가지고 있다면, 힙의 맨 위에 있는 요소는 중간에 있을 것이다. 마찬가지로, 데이터의 더 작은 절반만을 저장하는 최대 힙을 가지고 있다면, 힙의 맨 위에 있는 요소는 중간에 있을 것이다. 데이터 세트를 배열로 표현했다면, 왼쪽 반을 한 색으로, 오른쪽 반을 다른 색으로 색칠하는 것을 상상할 수 있다. 색깔은 각 힙의 범위를 나타내고, 그들은 중간에서 "만나게" 되며, 두 힙의 맨 위에 있는 값이다.

힙의 크기를 동일하게 유지하여 각 힙이 데이터의 절반을 보유하고 있도록 한다면, 요소의 수가 짝수인 경우, 중앙값은 두 힙의 맨 위에 있는 값들의 평균이 될 것이다. 요소의 수가 홀수인 경우, 한 힙은 다른 힙보다 하나 더 큰 크기를 갖게 될 것이다. 그 하나 더 많은 요소가 중앙값이다. 홀수일 때 어느 힙에 중앙값을 저장할지는 중요하지 않다 - 임의로 최대 힙을 선택하자.

![heap2](heap2.png)

힙에 추가할 때, 힙의 크기 차이가 동일하게 유지되도록 해야 한다(요소의 수가 홀수인 경우 1 내외). 또한 min heap의 **모든 원소가 max heap의 모든 원소보다 크거나 같도록 해야 한다**(그렇지 않으면 색칠 비유가 깨질 수 있다). 이를 달성하기 위해 다음과 같은 알고리즘을 사용할 수 있다:

1. `num`을 max heap에 푸시한다(max heap을 임의로 선택한 것처럼).
2. max heap에서 팝한 다음, 그 원소를 min heap에 푸시한다.
3. 2단계 후 min heap이 max heap보다 더 많은 원소를 가지면, min heap에서 팝하고 그 결과를 max heap에 푸시한다.

다시 말해, 데이터 집합을 다른 색으로 절반 나눈 배열로 상상해보자. 한 힙에서 팝한 다음 결과를 다른 힙에 푸시할 때, 색상 변경 지점을 한 원소만큼 이동시키는 것과 같다.

3단계는 요소의 수가 홀수일 경우 max heap이 추가 요소를 저장하도록 결정을 유지하는 방법이다.

> 2단계는 굵은 글씨체로 표시된 속성을 유지하는 방법이다. 데이터 세트 `1, 3, 7, 13, 36, 100`이 있고 `50`을 추가하려고 한다고 상상해보자. 1단계에서 max heap에 추가하면 max heap은 `50, 7, 3, 1`이 된다. min heap의 `13`과 `36`보다 크기 때문에 max heap에 속하지 않는다는 것에 주목하자. max heap에서 팝한 다음 min heap에 푸시하면 속성이 유지된다.
>
> 요소의 총 수가 홀수일 때 max heap을 원소가 더 많은 힙으로 선택했기 때문에, 결국 `13`을 끝에서 밀어내고 max heap에 넣게 된다. 모든 작업 후 max heap은 `13, 7, 3, 1`이 되고, 중앙값 `13`은 올바르게 위치하게 된다.
{: .prompt-general }

```cpp
class MedianFinder {
public:
    priority_queue<int, vector<int>, greater<int>> minHeap;
    priority_queue<int> maxHeap;
    
    MedianFinder() {}
    
    void addNum(int num) {
        maxHeap.push(num);
        minHeap.push(maxHeap.top());
        maxHeap.pop();
        
        if (minHeap.size() > maxHeap.size()) {
            maxHeap.push(minHeap.top());
            minHeap.pop();
        }
    }
    
    double findMedian() {
        if (maxHeap.size() > minHeap.size()) {
            return maxHeap.top();
        }
        
        return (minHeap.top() + maxHeap.top()) / 2.0;
    }
};
```

이 알고리즘은 `findMedian`에 대한 $$O(1)$$의 시간 복잡도와 `addNum`에 대한 $$O(\log{}n)$$의 시간 복잡도를 갖게 해주며, 여기서 $$n$$은 지금까지 `addNum`이 호출된 횟수다. 공간 복잡도는 힙을 저장하기 위한 $$O(n)$$이다.

힙은 해시 맵과 유사하고, 트리/그래프나 연결 리스트와는 다르게, 힙이 사용될 때 문제는 힙에만 집중되지 않는다. 힙은 대개 알고리즘에서 효율적으로 무언가를 달성하기 위한 도구로 사용된다. 다음 장(탐욕(Greedy) 알고리즘)에서는 힙에 대해 더 자세히 이야기하고, 효율적인 알고리즘을 구현하는 데 어떻게 도움을 줄 수 있는지에 대해 논의할 것이다. 그 전에, 다음 글에서는 힙을 사용하는 일반적인 패턴에 대해 이야기할 것이다.

---

출처: [Leetcode](https://leetcode.com/explore/interview/card/leetcodes-interview-crash-course-data-structures-and-algorithms/708/heaps/4649/)

<!--

{: .prompt-general }

-->