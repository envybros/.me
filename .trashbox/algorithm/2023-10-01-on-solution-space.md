---
title: "솔루션 공간에서 이진 탐색"
categories: [Algorithm 연구소]
tags: [Algorithm]
date: 2023-10-01 01:30
math: true
img_path: /assets/img/algorithm/
---

---

![Title](algorithm_title.png)

---

## **개요**

이진 탐색을 더 창의적으로 사용하는 방법이 있는데, 바로 솔루션 공간이나 정답에 적용하는 것이다. 매우 일반적인 문제 유형 중 하나는 "어떤 것을 할 수 있는 최대/최소 값은 무엇인가"이다. 다음과 같은 기준이 충족되면 이진 탐색을 사용할 수 있다:

1. 주어진 숫자 `x`에 대해 작업이 가능한지를 $$O(n)$$ 이하의 복잡도로 빠르게 확인할 수 있다.
2. 숫자 `x`에 대해 작업이 가능한 경우, 찾고 있는 것이:
    - 최대값이면, `x`보다 작은 모든 숫자에 대해서도 가능하다.
    - 최소값이면, `x`보다 큰 모든 숫자에 대해서도 가능하다.
3. 숫자 `x`에 대해 작업이 불가능한 경우, 찾고 있는 것이:
    - 최대값이면, `x`보다 큰 모든 숫자에 대해서도 불가능하다.
    - 최소값이면, `x`보다 작은 모든 숫자에 대해서도 불가능하다.

2번째와 3번째 요구사항은 두 개의 "구역(zone)"이 있다는 것을 의미한다. 하나는 가능한 구역이고 다른 하나는 불가능한 구역이다. 이 구역들은 간격 없이, 겹침 없이, 한계점에 의해 분리되어 있다.

![on_solution](on_solution.png)

문제가 최소/최대 값을 찾으라고 할 때, 불가능에서 가능으로 전환되는 한계점을 찾으라는 것이다.

먼저 최소 가능한 답과 최대 가능한 답을 식별하여 가능한 솔루션 공간을 설정한다.

다음으로, 이 솔루션 공간에 이진 탐색을 실시한다. 각 `mid`에 대해 해당 작업이 가능한지 확인한다. 결과에 따라 탐색 공간을 절반으로 줄인다. 결국에는 한계점을 찾게 된다.

첫 번째 요구사항이 충족된다면 (`mid`가 가능한지 빠르게 확인), 시간 복잡도는 $$O(n \cdot \log{}k)$$가 될 것이다. 여기서 $$k$$는 솔루션 공간의 범위다. 솔루션 공간이 아무리 크더라도 로그는 빠르게 실행되므로 이는 매우 효율적인 시간 복잡도다.

해당 정수에 대해 작업이 가능한지 확인하는 함수 `check`를 작성할 수 있다. 대부분의 경우 이 함수에서 사용하는 알고리즘은 탐욕 알고리즘이 될 것이다. 예를 들어 보자.

---

## **예제 1: 바나나 먹는 코코**

> [문제 링크](https://leetcode.com/problems/koko-eating-bananas)
>
> 코코는 바나나를 먹는 것을 좋아한다. 바나나 더미가 `n`개 있고, $$i$$번째 더미에는 `piles[i]` 바나나가 있다. 코코는 시간당 `k`개의 바나나를 먹을 속도를 정할 수 있다. 매시간마다 그녀는 더미를 선택하고 그 더미에서 `k`개의 바나나를 먹는다. 더미에 `k`보다 적은 바나나가 있으면 그녀는 모두 먹고 그 시간 동안 더 이상 바나나를 먹지 않는다. 그녀가 `h` 시간 내에 모든 바나나를 먹을 수 있는 최소 정수 `k`를 반환한다.
{: .prompt-general }

먹는 속도 `k`가 있고 그 작업이 가능한 경우, `k`보다 큰 모든 먹는 속도에서도 작업이 가능하다. 만약 가능하지 않다면, `k`보다 느린 모든 먹는 속도로는 작업을 수행할 수 없다. 이는 우리가 답을 이진 탐색으로 찾을 수 있음을 의미한다.

먹는 속도 `k`가 주어졌을 때, 모든 더미를 `h` 시간 내에 먹을 수 있는지 어떻게 확인할 수 있을까? 코코는 한 시간에 하나의 더미에서만 먹을 수 있기 때문에, 우리는 더미를 개별적으로 살펴볼 수 있다. 더미에 바나나가 있다면, 그 더미를 먹는데 얼마나 걸릴까? `k`와 관계없이 바나나 한 개가 남아 있어도 추가 시간이 걸린다. 이는 `bananas / k`의 시간이 소요된다는 것을 의미한다(올림 처리). 우리는 더미들을 순회하면서 각 더미에 대해 `bananas / k`의 합을 찾고, 그것이 `h`보다 작거나 같은지 확인할 수 있다.

이진 탐색을 위해, 경계는 최소 답과 최대 답에서 시작해야 한다. 가능한 최소 답은 `1`이다 - 코코는 시간당 `0`개 이상의 바나나를 먹어야 한다! 가능한 최대 답은 `max(piles)`이다. 이보다 더 빠른 먹는 속도를 가질 필요는 없다.

### [예제 1] 상세 설명

이진 탐색의 사용 및 구현을 파악하기 위한 여러 단계가 있다. 먼저, 문제는 최소 먹는 속도를 요구한다. 코코가 `k`의 속도로 모든 바나나를 먹을 수 있다면, 그녀는 확실히 `k + 1`의 속도로도 모든 바나나를 먹을 수 있다. 그녀가 할 수 없다면, 확실히 `k - 1`의 속도로도 못 할 것이다.

이렇게 두 개의 "존"이 생성된다. 다음으로, 탐색 공간 경계를 식별해야 한다. 입력이 무엇이든 간에, 먹는 속도는 절대 `1`보다 낮을 수 없다. 왜냐하면 먹는 속도는 정수여야 하고, `0`의 먹는 속도는 바나나가 전혀 먹히지 않는다는 것을 의미하기 때문이다.

먹는 속도가 `max(piles)`이면, 각 더미는 한 시간이 걸린다. 이것을 개선할 수 있는 방법은 없다(문제는 더미에 있는 바나나의 수보다 먹는 속도가 더 클지라도 여전히 한 시간이 걸린다고 명시하고 있다).

따라서 우리의 탐색 공간은 `[1, max(piles)]`이다. 마지막으로, 주어진 먹는 속도 `k`에 대한 `check` 함수를 어떻게 구현할지 파악해야 한다. 문제 설명에 따르면 더미에 `k`보다 적은 바나나가 있으면 코코는 그것들을 모두 먹고 나머지 시간 동안 더 이상 먹지 않는다. 더미에 바나나 `8`개가 있고 코코의 먹는 속도가 3이라면, 코코는 `2`시간에 바나나 `6`개를 먹을 수 있고, 더미에는 바나나 `2`개가 남는다. 그런 다음 코코는 남은 `2`개의 바나나를 먹는데 추가 시간을 보내야 한다.

k의 먹는 속도로 바나나 더미를 먹는 데 필요한 시간은 `bananas / k`이며, 이는 올림해야 한다. 우리는 입력을 반복 처리하면서 `bananas / k`의 값을 합산하고, 그것이 제한 시간 내에 있는지 확인할 수 있다. 탐색 공간을 식별하고 `check` 함수를 구현하면, 우리는 이진 탐색을 수행하여 임계값을 찾을 수 있다.

```cpp
class Solution {
public:
    int limit;
    
    int minEatingSpeed(vector<int>& piles, int h) {
        limit = h;
        int left = 1;
        int right = 0;
        for (int bananas: piles) {
            right = max(right, bananas);
        }
        
        while (left <= right) {
            int mid = left + (right - left) / 2;
            if (check(mid, piles)) {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        
        return left;
    }
    
    bool check(int k, vector<int>& piles) {
        long hours = 0;
        for (double bananas: piles) {
            hours += ceil(bananas / k);
        }
        
        return hours <= limit;
    }
};
```

`check` 함수는 $$O(n)$$ 시간에 실행되며, 여기서 `n = piles.length`이다. 이진 탐색은 $$O(\log{}k)$$ 시간에 실행되는데, 여기서 `k = max(piles)`이다. 이로 인해 우리는 시간 복잡도 $$O(n \cdot \log{}k)$$를 가지게 되는데, $$\log$$ 함수 때문에 이것은 매우 빠르다. 몇 개의 정수 변수를 제외하고 추가 공간을 사용하지 않는다.

---

## **예제 2: 최소 노력으로 가능한 경로**

> [문제 링크](https://leetcode.com/problems/path-with-minimum-effort/)
>
> 당신은 `heights`라는 `m x n` 크기의 양의 2D 배열을 받는다. 여기서 `heights[row][col]`은 셀 `(row, col)`의 높이를 나타낸다. 당신은 위, 아래, 왼쪽 또는 오른쪽으로 이동할 수 있다. 경로의 노력은 건너간 연속된 두 셀 사이에 있을 수 있는 가장 큰 절대 차이다. 왼쪽 상단에서 오른쪽 하단까지 가는 데 필요한 최소 노력(`effort`)을 반환하라.
{: .prompt-general }

`effort`으로 여행을 할 수 없다면, `effort`보다 작은 어떤 숫자로도 그것을 할 수 없다. 할 수 있다면, `effort`보다 큰 모든 숫자로 그것을 할 수 있다.

정수 `effort`가 주어지면, 경로가 존재하는지 어떻게 확인할 수 있을까? 우리는 간단한 DFS를 `(0, 0)`에서 시작할 수 있으며, 간선은 4 방향에 있고 현재 노드와 다음 노드 사이의 차이가 `effort` 이하인 경우에만 이동할 수 있다.

이진 탐색을 위해, 경계는 어디서 시작해야 할까? 가능한 최소 `effort`는 `0`이다 - 모든 숫자가 같은 경로가 존재할 수 있다. 가능한 최대 `effort`는 입력값에서 가장 큰 숫자인데, 입력값에는 음수가 없기 때문이다.

### [예제 2] 상세 설명

이전 예제와 같은 단계를 거쳐 보자. 먼저 문제는 최소 노력을 요구한다. 주어진 `effort`에 대해 작업이 가능하다면, 모든 인접한 셀이 `effort` 이하의 차이를 가진 어떤 경로가 있음을 의미한다.

확실히, 우리가 말하는 경로도 최대 허용 차이를 늘리면 통과할 수 있으므로 `effort + 1`로 작업이 가능하다. 마찬가지로, 주어진 `effort`에 대해 작업이 불가능하다면 `effort - 1`로는 확실히 불가능할 것이다. 경로가 없다면, 최대 허용 차이를 낮추는 것은 경로를 생성하지 않는다. 우리는 두 영역을 식별했다. 다음으로, 탐색 공간을 식별해야 한다. 임의의 입력이 주어지면, 작업은 `0` 노력으로 가능할 수 있다(예를 들어, 모든 칸이 같은 값을 가진 경우). 입력값에서 가장 큰 숫자가 `x`라면, 우리는 `x`보다 많은 노력을 필요로 하지 않을 것이다.

음수는 허용되지 않으므로, `x`보다 큰 두 요소 사이의 차이가 존재할 수 없다. 우리는 탐색 공간을 `[0, x]`로 식별했다. 마지막으로, 주어진 노력에 대한 `check` 함수를 구현해야 한다. 우리는 트리와 그래프 장에서 여러 번 했던 것처럼 행렬을 그래프로 모델링할 수 있다. 왼쪽 상단의 사각형에서 DFS를 수행하고 오른쪽 하단의 사각형에 도달하려고 시도한다. 노드 사이의 차이가 `effort` 이하인 경우에만 이웃으로 이동한다.

{% raw %}

```cpp
class Solution {
public:
    int m;
    int n;
    vector<vector<int>> directions = {{0, 1}, {1, 0}, {0, -1}, {-1, 0}};
    
    int minimumEffortPath(vector<vector<int>>& heights) {
        m = heights.size();
        n = heights[0].size();
        int left = 0;
        int right = 0;
        for (vector<int>& row: heights) {
            for (int num: row) {
                right = max(right, num);
            }
        }
        
        while (left <= right) {
            int mid = left + (right - left) / 2;
            if (check(mid, heights)) {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        
        return left;
    }
    
    bool check(int effort, vector<vector<int>>& heights) {
        vector<vector<bool>> seen(m, vector<bool>(n, false));
        stack<pair<int, int>> stack;
        seen[0][0] = true;
        stack.push(pair(0, 0));
        
        while (!stack.empty()) {
            auto [row, col] = stack.top();
            stack.pop();
            if (row == m - 1 && col == n - 1) {
                return true;
            }
            
            for (vector<int>& direction: directions) {
                int nextRow = row + direction[0], nextCol = col + direction[1];
                if (valid(nextRow, nextCol) && !seen[nextRow][nextCol]) {
                    if (abs(heights[nextRow][nextCol] - heights[row][col]) <= effort) {
                        seen[nextRow][nextCol] = true;
                        stack.push(pair(nextRow, nextCol));
                    }
                }
            }
        }
        
        return false;

    }
    
    bool valid(int row, int col) {
        return 0 <= row && row < m && 0 <= col && col < n;
    }
};
```

{% endraw %}

DFS는 $$O(m \cdot n)$$ 시간이 걸리며, 이진 탐색은 최대 요소가 입력된 k에 대해 $$O(\log{}k)$$ 번 실행된다. 이로 인해 시간 복잡도는 $$O(m \cdot n \cdot \log{}k)$$가 된다. DFS를 수행하기 위해 스택과 seen에 $$O(m \cdot n)$$ 공간이 사용된다.

---

## **예제 3: 시간 내에 도착하기 위한 최소 속도**

> [문제 링크](https://leetcode.com/problems/minimum-speed-to-arrive-on-time/)
>
> float 형의 `hour`가 주어지며, 이는 사무실에 도착해야 하는 시간을 나타낸다. 사무실로 통근하기 위해, 연속으로 `n`개의 기차를 타야 한다. 또한 정수 배열 `dist`가 주어지며, 여기서 `dist[i]`는 $$i$$번째 기차 여행의 거리를 설명한다. 각 기차는 정수 시간에만 출발할 수 있으므로, 각 기차 여행 사이에 기다려야 할 수도 있다.
>
> 예를 들어, 첫 번째 기차 여행에 1.5시간이 걸린다면, 2시간 표시에서 두 번째 기차 여행을 시작하기 전에 추가로 0.5시간 기다려야 한다. 시간 내에 사무실에 도착하기 위해 모든 기차가 여행해야 하는 최소 양의 정수 속도를 반환하거나, 시간 내에 불가능한 경우 `-1`을 반환한다. 정답은 10^7을 초과하지 않는다.
{: .prompt-general }

이 문제는 "바나나 먹는 코코"와 상당히 유사하다. 속도 `k`가 있고 기차의 거리가 `d`라면 기차는 `d / k` 시간이 걸린다. 문제에는 기차가 정수 시간에만 출발한다는 추가 제약 조건이 있으므로, 각 기차를 타기 전에 시간을 반올림해야 한다.

가능한 최소 속도는 `1`이고, 최대는 문제 설명에 주어진 $$10^7$$이다. 문제가 이 정보를 제공하는 것은 실제로 이진 탐색을 사용하는 것을 향한 힌트지만, 좋은 질문을 제기한다 - 입력된 내용에서 최대 가능한 정답을 확정할 수 없을 때 무엇을 해야 하는가? `right`에 임의의 큰 숫자를 사용할 수 있다, 예를 들어 $$10^10$$. 로그는 매우 빠르기 때문에 거의 차이가 나지 않는다.

작업은 일반적으로 불가능할 수도 있다 - 문제가 불가능한 경우 `-1`을 반환해야 한다고 말한다. 언제 불가능한가? 각 기차는 정수에서만 출발하기 때문에, 우리가 매우 높은 속도를 가지고 있더라도 각 기차는 적어도 1시간이 걸린다. 따라서 허용된 시간보다 기차가 더 많은 경우, 그것은 불가능하다.

> 참고: `check` 함수에서는 최종 값을 올림하지 않는다 왜냐하면 더 이상 기차를 기다릴 필요가 없기 때문이다. 올림은 다음 기차를 다음 정수 시간에 기다리기 위해 시뮬레이션하는 것이다.
{: .prompt-general }

```cpp
class Solution {
public:
    double limit;
    
    int minSpeedOnTime(vector<int>& dist, double hour) {
        if (dist.size() > ceil(hour)) {
            return -1;
        }
        
        limit = hour;
        int left = 1;
        int right = pow(10, 7);
        
        while (left <= right) {
            int mid = left + (right - left) / 2;
            if (check(mid, dist)) {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        
        return left;
    }
    
    bool check(double k, vector<int>& dist) {
        double t = 0;
        for (double d: dist) {
            t = ceil(t);
            t += d / k;
        }
        
        return t <= limit;
    }
};
```

기술적으로 이 문제는 시간 복잡도가 $$O(n)$$이며, 여기서 n은 입력 배열의 길이다. 왜냐하면 문제에서 최대 답이 $$10^7$$이라고 명시적으로 말해주고, 이의 로그는 상수이기 때문이다. 그러나 알고리즘이 시간 복잡도가 $$O(n \cdot \log{}k)$$라고 말하는 것이 더 적절하다. 여기서 $$k$$는 가능한 최대 답이다. 몇 개의 정수 변수를 제외하고 추가 공간을 사용하지 않는다.

---

## **구현에 대한 참고 사항**

이 문서에서 살펴본 모든 3개의 예제는 최소값을 요구한다. 모든 해결책에서, 우리는 `left`를 반환한다.

문제가 대신 최대값을 요구하는 경우, `left`는 실제로 마지막에 정확한 정답이 아니다. 대신, 우리는 `right`를 반환해야 한다.

> 최소값을 찾을 때 `left`가 정답을 가리키는 이유는 무엇이고, 최대값을 찾을 때는 `right`가 정답을 가리키는 이유는 무엇인가?
{: .prompt-general }

최소값을 찾고 있다고 가정하고 정답이 `x`라고 하자. `check(x)`를 수행한 후, 우리는 `right = x - 1`로 설정한다. 왜냐하면 `check(x)`는 `true`를 반환할 것이고, 우리는 더 나은 정답을 찾기 위해 오른쪽 경계를 이동시킬 것이기 때문이다. 보다시피, 정확한 정답은 이제 우리의 탐색 공간 밖에 실제로 있다. 이는 `check`의 모든 미래의 반복이 실패할 것이라는 것을 의미하며, 결국 우리는 `check(x - 1)`을 시도할 때까지 `left`를 계속 증가시킬 것이다. 이것은 실패하고 `left = (x - 1) + 1 = x`를 설정한다. 우리의 while 루프는 `left > right` 때문에 종료되고, `left`는 정답에 있다.

대신 최대값을 찾고 있다면, `check(x)`를 수행한 후에 `left = x + 1`을 설정한다. 또한, 정확한 정답은 탐색 공간 밖에 있고 모든 미래의 체크가 실패할 것이다. 결국, 우리는 `check(x + 1)`을 시도하고, 실패하며, `right = (x + 1) - 1 = x`를 설정한다. 루프는 `right < left` 때문에 종료되고, `right`는 정답을 가리킨다.

## **보너스 문제**

### 배열에서 이진 탐색

- [374. Guess Number Higher or Lower](https://leetcode.com/problems/guess-number-higher-or-lower/)
- [1855. Maximum Distance Between a Pair of Values](https://leetcode.com/problems/maximum-distance-between-a-pair-of-values/)
- [2476. Closest Nodes Queries in a Binary Search Tree](https://leetcode.com/problems/closest-nodes-queries-in-a-binary-search-tree/)
- [2250. Count Number of Rectangles Containing Each Point](https://leetcode.com/problems/count-number-of-rectangles-containing-each-point/)
- [1533. Find the Index of the Large Integer](https://leetcode.com/problems/find-the-index-of-the-large-integer/)
- [540. Single Element in a Sorted Array](https://leetcode.com/problems/single-element-in-a-sorted-array/)
- [33. Search in Rotated Sorted Array](https://leetcode.com/problems/search-in-rotated-sorted-array/)
- [2251. Number of Flowers in Full Bloom](https://leetcode.com/problems/number-of-flowers-in-full-bloom/)

### 솔루션 공간에서 이진 탐색

- [2187. Minimum Time to Complete Trips](https://leetcode.com/problems/minimum-time-to-complete-trips/)
- [2226. Maximum Candies Allocated to K Children](https://leetcode.com/problems/maximum-candies-allocated-to-k-children/)
- [2517. Maximum Tastiness of Candy Basket](https://leetcode.com/problems/maximum-tastiness-of-candy-basket/)
- [2141. Maximum Running Time of N Computers](https://leetcode.com/problems/maximum-running-time-of-n-computers/)
- [1482. Minimum Number of Days to Make m Bouquets](https://leetcode.com/problems/minimum-number-of-days-to-make-m-bouquets/)
- [778. Swim in Rising Water](https://leetcode.com/problems/swim-in-rising-water/)
- [1970. Last Day Where You Can Still Cross](https://leetcode.com/problems/last-day-where-you-can-still-cross/)
- [2258. Escape the Spreading Fire](https://leetcode.com/problems/escape-the-spreading-fire/)

---

출처: [Leetcode](https://leetcode.com/explore/interview/card/leetcodes-interview-crash-course-data-structures-and-algorithms/710/binary-search/4533/)

<!--

{: .prompt-general }

-->