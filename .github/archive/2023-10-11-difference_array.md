---
title: "차이 배열"
categories: [Algorithm 연구소]
tags: [Algorithm]
date: 2023-10-11 01:30
math: true
img_path: /assets/img/algorithm/
---

---

![Title](algorithm_title.png)

---

## **개요**

차이 배열은 매우 흔하게 사용되는 것은 아니지만, 면접에서 즉석에서 이 아이디어를 도출하기는 어려우므로 알아두면 좋다.

차이 배열은 이벤트 간격과 관련된 문제를 해결할 때 사용된다. 모든 이벤트가 숫자 선을 따라 발생하는 것으로 생각할 수 있다. 이 숫자 선은 시간 또는 위치를 나타낼 수 있다. 이벤트는 이 숫자 선에서 시작점과 끝점을 가지고 발생한다.

입력은 각 내부 배열이 `[left, right, value]` 형식인 2D 배열을 제공하거나, 일부 동등한 형태를 제공할 것이다. 문제의 배경은 대개 "`left`과 `right` 사이에 어떤 `value`이 있다"와 같은 식일 것이다. 몇 가지 예를 살펴보자.

---

## **예제 1: Car Pooling**

> [문제 링크](https://leetcode.com/problems/car-pooling/)
>
> 차에는 `capacity` 만큼의 승객을 태울 수 있는 공간이 있고, `trips` 배열이 주어진다. 각 여행은 `[numPassengers, from, to]`로 표현되며, `from`에서 `numPassengers`를 태우고 `to`에서 그들을 내린다는 것을 나타낸다. 차가 언제든 `capacity`를 초과하지 않고 모든 여행을 완료할 수 있을까?
{: .prompt-general }

이 문제에서 숫자 라인은 도로다. 각 여행은 이벤트이며 `from` 위치에서 숫자 라인에 나타나고 `to` 위치에서 숫자 라인을 떠난다. 이벤트 동안 `numPassengers`가 차를 차지한다. 위에서 언급한 입력 형식과 관련하여, 우리는 `from`을 왼쪽으로, `to`를 오른쪽으로, 그리고 `numPassengers`를 값으로 갖는다.

우리가 할 수 있는 일은 배열로 숫자 라인을 구축하는 것이다. 먼저, `to`의 최대값, 즉 가장 먼 위치를 찾아야 한다. 그런 다음, `farthest + 1` 길이로 도로를 배열 `arr`로 초기화할 수 있다.

이제, 각 이벤트에 대해 `from` 위치에서 `numPassengers`가 차에 탑승한다는 것을 알고 있다. 따라서 `arr[from] += numPassengers`를 수행할 수 있다. `to` 위치에서는 `numPassengers`가 차에서 내릴 것이므로, `arr[to] -= numPassengers`를 수행할 수 있다.

모든 이벤트를 처리한 후, `arr`의 값은 각 위치에서 승객 수의 변화를 나타낼 것이다. 이제, 도로에서 간단한 누적 합을 수행하고 어느 시점에서든 승객 수(누적 합)가 `capacity`를 초과하는지 확인하면 된다.

```cpp
class Solution {
public:
    bool carPooling(vector<vector<int>>& trips, int capacity) {
        int farthest = 0;
        for (vector<int>& trip: trips) {
            farthest = max(farthest, trip[2]);
        }
        
        vector<int> arr(farthest + 1);
        for (vector<int>& trip: trips) {
            int value = trip[0], left = trip[1], right = trip[2];
            arr[left] += value;
            arr[right] -= value;
        }
        
        int curr = 0;
        for (int i = 0; i < arr.size(); i++) {
            curr += arr[i];
            if (curr > capacity) {
                return false;
            }
        }
        
        return true;
    }
};
```

이 알고리즘은 시간 복잡도와 공간 복잡도가 $$O(n + m)$$이다. 여기서 `m`은 `trips` 내의 `trip[2]`의 `최대값`이다. 이 방법 없이는 여행을 정렬해야 하며, 이 경우 시간 복잡도가 $$O(n \cdot \log{}n)$$이 된다.

차이 배열 방법은 각 위치에서 승객 수의 변화를 나타내는 이 배열을 구축한 다음 누적 합을 사용하는 것이다.

---

입력 형식이 `[left, right, value]` 또는 동등한 형태여야 한다고 말했다. 동등한 형태란 무엇을 의미하는가?

## **예제 2: 가장 밝은 위치**

> 당신은 지금 가로등이 있는 거리에 있다. `lights` 배열로 표현된다. 각 불빛은 `[position, radius]`으로 주어지며, 빛이 `position`에 있고 `radius` 거리만큼 좌우로 비춘다는 것을 의미한다. 거리에서 가장 밝은 곳을 가장 많은 불빛이 비추는 곳이라고 하자. 그러한 위치를 반환해보자.
>
> 거리가 **매우 길다는 것**을 주의해야 한다 - `position <= 10^18`.
{: .prompt-general }

여기 동등한 형태의 예가 있다. 문제에서 불빛의 밝기가 다르다고 말하지 않았으므로 모든 불빛의 `value`는 `1`이다. `left`은 `position - radius`로 계산할 수 있고, `right`은 `position + radius`로 계산할 수 있다.

문제에서 거리가 매우 길다고 언급했기 때문에, 첫 번째 예제에서 우리가 "건물"을 만들고 그것을 따라 걷는 것과 같은 방법을 사용하는 것은 비용이 너무 많이 들 것이다. 대신, 각 요소가 `[position, value]`인 배열 `change`를 만들 수 있다. 여기서 밝기는 `position`에서 `value`만큼 변한다. 여기서의 논리는 원래 방법과 동일하다 - 우리가 빛의 범위 `left`에 들어갈 때, 밝기는 `value`만큼 증가한다. 우리가 빛의 범위 `right`을 벗어날 때, 밝기는 `value`만큼 감소한다. 그런 다음 배열을 정렬하고 접두사 합으로 그것을 따라 걸을 수 있다.

```cpp
int findBrightestPosition(vector<vector<int>>& lights) {
    vector<pair<int, int>> change;
    for (vector<int>& light: lights) {
        int position = light[0], radius = light[1];
        change.push_back({position - radius, 1});
        change.push_back({position + radius + 1, -1});
    }
    
    sort(change.begin(), change.end());
    int ans = 0, curr = 0, brightest = 0;
    for (auto [position, value]: change) {
        curr += value;
        if (curr > brightest) {
            brightest = curr;
            ans = position;
        }
    }
    
    return ans;
}
```

첫 번째 예제에서 언급했듯이, 정렬함으로써 시간 복잡도는 $$O(n \cdot \log{}n)$$이 된다. 이것은 거리가 매우 길기 때문에 필요하다 - 첫 번째 방법을 사용하는 것은 `position`가 최대 $$10^{18}$$이 될 수 있기 때문에 너무 느리다.

방법에 대한 첫 번째 또는 두 번째 접근 방식을 사용할 때를 알아야 한다. "걷기"에 대한 논리는 매우 유사하다 - "건물" 부분만 다르고, 올바른 결정은 문제의 제약 조건에 따라 달라질 것이다.

---

출처: [Leetcode](https://leetcode.com/explore/interview/card/leetcodes-interview-crash-course-data-structures-and-algorithms/714/bonus/4688/)

<!--

{: .prompt-general }

-->