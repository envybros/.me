---
title: "탐욕 문제들 예시"
categories: [Algorithm 연구소]
tags: [Algorithm]
date: 2023-09-29 01:30
math: true
img_path: /assets/img/algorithm/
---

---

![Title](algorithm_title.png)

---

## **예제 1: 소행성 파괴**

> [문제 링크](https://leetcode.com/problems/destroying-asteroids)
>
> 정수 배열인 `asteroids`와 행성의 질량을 나타내는 정수 `mass`가 주어진다. 행성은 하나씩 소행성과 충돌할 것이며 - 순서를 선택할 수 있다. 행성의 질량이 소행성의 질량보다 작으면 행성이 파괴된다. 그렇지 않으면 행성은 소행성의 질량을 얻는다. 모든 소행성을 파괴할 수 있을까?
{: .prompt-general }

행성이 소행성과 충돌하는 최적의 순서를 선택해야 한다. 시작할 때, 행성보다 질량이 작거나 같은 소행성이 두 개, `x`와 `y`만 있다고 가정해보자. 여기서 `x < y < planet`이다. 먼저 어떤 소행성과 충돌해야 할까? 정답은 상관없다는 것이다. 왜냐하면 우리의 질량은 오직 증가할 수 있기 때문에, 어느 시점에서든 소행성을 파괴할 수 있다면, 우리는 항상 그 소행성을 미래에 파괴할 수 있다. 주어진 단계에서 어떤 소행성을 가져갈 수 있는지 가장 쉽게 알아내는 방법은 무엇일까?

파괴할 수 있는 소행성을 유지하기 위해 입력값을 정렬하고 순회할 수 있다. 그런 다음, 각 단계에서 질량이 가장 작은 소행성을 탐욕적으로 선택하기만 하면 된다. 어떤 단계에서든 남아 있는 가장 작은 소행성의 질량이 우리 행성보다 크다면, 모든 소행성을 파괴하는 것은 불가능하고 어떤 소행성의 순서도 우리가 계속 진행할 수 있게 해주지 않는다.

> 정확한 솔루션으로 이어지는 여러 전략이 있을 수 있지만, 매 단계에서 남아 있는 가장 작은 소행성을 탐욕적으로 선택하는 것은 직관적이고 구현하기 쉬운 전략이다.
{: .prompt-general }

```cpp
class Solution {
public:
    bool asteroidsDestroyed(int mass, vector<int>& asteroids) {
        sort(asteroids.begin(), asteroids.end());
        long currMass = mass;
        for (int asteroid: asteroids) {
            if (asteroid > currMass) {
                return false;
            }
            
            currMass += asteroid;
        }
        
        return true;
    }
};
```

이 예에서 볼 수 있듯이, 가장 작은 소행성을 탐욕적으로 선택하는 것이 최적의 방법임을 깨닫고 나면 알고리즘을 구현하는 것은 매우 간단하다. 이 알고리즘은 정렬 때문에 $$O(n \cdot \log{}n)$$의 시간 복잡도를 가지며, 여기서 n은 소행성의 수다. 사용되는 공간의 양은 사용하는 언어에 따라 다르다. 예를 들어, Python은 최대 $$O(n)$$ 공간을 사용하는 timsort를 구현하지만, quicksort를 사용하는 언어는 $$O(\log{}n)$$ 공간을 사용한다.

---

## **예제 2: 최대 차이가 K인 배열 분할**

> [문제 링크](https://leetcode.com/problems/partition-array-such-that-maximum-difference-is-k/)
>
> 정수 배열 `nums`와 정수 `k`가 주어졌을 때, `nums`를 서브시퀀스로 분할하되, 각 서브시퀀스의 최대 및 최소 요소가 서로 `k` 이내에 있어야 한다. 필요한 서브시퀀스의 최소 개수는 무엇인가?
>
> 예를 들어, `nums = [3, 6, 1, 2, 5]` 및 `k = 2`인 경우, 정답은 `2`다. 서브시퀀스는 `[3, 1, 2]`와 `[6, 5]`이다.
{: .prompt-general }

우리는 배열과 문자열 챕터에서 잠시 서브시퀀스에 대해 얘기했었는데, 다시 상기시키자면, 서브시퀀스는 배열에서 순서를 유지하는 요소 그룹이다. 이들은 서브어레이와 비슷하지만 요소들이 인접할 필요는 없다.

이 문제에서, 각 서브시퀀스에 대해 우리가 신경 쓸 것은 최대값과 최소값뿐이다. 서브시퀀스 내 요소들의 실제 순서는 중요하지 않다. 순서 요구사항이 없다면, 서브시퀀스는 서브셋과 같다. 우리는 간단히 서브셋에 대해서도 얘기했었는데, 서브셋은 배열에서의 요소 집합에 불과하다.

요소들을 그룹핑하는 최선의 방법은 무엇일까? 그룹의 수를 최소화하고 싶기 때문에, 그룹 내 요소의 수를 최대화하고 싶다. 가장 작은 숫자 x로 시작한다고 가정해보자. 우리는 범위 `[x, x + k]` 내의 모든 요소가 그룹핑되길 원한다. 이 요소들 중 어느 것도 그룹에 포함되지 않는 것은 말이 되지 않는다 - `x + k - 1`을 제외하기로 결정했다고 가정해보자. 그러면 다음 그룹은 더 작은 숫자에서 시작해야 한다. 이는 그룹핑할 수 있는 요소의 범위를 제한하는 것으로, 우리가 원하는 것에 반한다.

따라서, 가장 작은 숫자 `x`에 대해 범위 `[x, x + k]` 내의 모든 요소를 탐욕적으로 가져가는 것이 최선이다. 그 후에, 그 숫자들을 배열에서 "지울" 수 있고, 우리는 다른 `x`로 같은 문제를 다시 가지게 된다. 이것을 실행하는 최선의 방법은 무엇일까? 배열을 정렬하고 순회한다. 우리가 논리적으로 서브시퀀스를 서브셋으로 줄였기 때문에, 정렬은 아무 것도 바꾸지 않는다.

### [예제 2] 상세 내용

`nums = [3, 6, 1, 2, 5]`와 `k = 2`라고 가정해보자. 최적의 서브시퀀스는 `[3, 1, 2]`와 `[6, 5]`다. 입력값을 정렬하면 어떻게 될까? 그러면 `[1, 2, 3, 5, 6]`이 된다. 우리는 요소들의 순서를 완전히 망쳐버렸고, 두 최적의 서브시퀀스는 더 이상 존재하지 않는다. 그러나 이게 문제일까? `[3, 1, 2]`와 정렬된 입력값에서 형성될 수 있는 새로운 서브시퀀스 `[1, 2, 3]` 사이에 의미 있는 차이가 있을까? 없다. 왜냐하면 둘 다 우리가 관심을 갖는 것은 최대 요소와 최소 요소뿐이며, 이는 순서와 무관하기 때문이다. 따라서 우리는 걱정 없이 입력값을 정렬할 수 있다. 가장 작은 요소인 `1`은 그룹에 속해야 한다. `2`와 `3`을 같은 그룹에 넣을 수 있다. 왜냐하면 그들은 `k` 이내이기 때문이다. 그렇게 해야 할까? 만약 그렇게 한다면, 우리는 `answer`를 하나 증가시키고, 남은 문제 `[5, 6]`를 해결해야 한다. 3을 제외한다면, `answer`를 하나 늘려야 하고, 그 다음 남은 문제 `[3, 5, 6]`를 해결해야 한다. `2`와 `3`을 모두 제외한다면, 우리는 `answer`를 하나 늘려야 하고, 그 다음 남은 문제 `[2, 3, 5, 6]`을 해결해야 한다. 모든 3가지 경우에, 우리는 `answer`를 하나씩 늘린다. 따라서, 우리는 남은 문제가 가장 작은 경우를 선택할 수도 있다. 왜냐하면 우리는 `answer`를 최소화하고 싶기 때문이다. 입력값을 정렬하면 그룹당 가능한 한 많은 숫자를 탐욕적으로 가져가는 것이 최적의 전략이라는 결론을 내릴 수 있다. 이것이 전략의 정확성에 대한 공식적인 수학적 증명은 아니지만, 면접에서 충분한 설명이 될 것이다.

```cpp
class Solution {
public:
    int partitionArray(vector<int>& nums, int k) {
        sort(nums.begin(), nums.end());
        int ans = 1;
        int x = nums[0];
        
        for (int i = 1; i < nums.size(); i++) {
            if (nums[i] - x > k) {
                x = nums[i];
                ans++;
            }
        }
        
        return ans;
    }
};
```

요약하자면, 시작할 때 `x`를 설정하고 가능한 많은 요소를 가져가야 한다. `x + k`를 넘어서면 정답을 증가시키고 새로운 `x`로 다시 시작한다. 이 방법은 $$O(n \cdot \log{}n)$$의 시간이 걸리는데, 여기서 $$n$$은 입력 배열의 길이이고, 정렬 때문에 이런 시간이 걸린다. 다시 말해, 정렬하는 동안 사용되는 추가 공간만 있을 뿐이고, 복잡도는 선택한 언어에 사용된 정렬 알고리즘에 달려 있다.

---

## **예제 3: IPO**

> [문제 링크](https://leetcode.com/problems/ipo/)
>
> 회사에서 `IPO` 전에 자본을 늘리기 위한 몇몇 프로젝트를 진행하고 싶어한다. 당신은 `n`개의 프로젝트를 받게 되는데, $$i^{th}$$ 프로젝트는 `profits[i]`의 이익과 `capital[i]`의 최소 자본이 필요하다. 처음에는 `w` 자본을 가지고 있다. 프로젝트를 마치면, 이익이 총 자본에 추가된다. `k`개의 프로젝트를 할 수 있을 때, 가능한 최대 자본을 반환해라.
{: .prompt-general }

우리는 최대 `k`개의 프로젝트를 선택할 수 있다. 어떤 것을 선택해야 할까? 첫 번째 프로젝트로, 초기 자본 `w`로 감당할 수 있는 모든 프로젝트 중에서 가장 많은 이익을 내는 프로젝트를 해야 한다. 정답이 최대 자본을 원하기 때문이고, 이것은 다음 결정을 위해 더 많은 프로젝트에 대한 문을 열어 준다. 이 로직은 모든 단계에서 적용할 수 있다 - 현재 감당할 수 있는 프로젝트 중에서 가장 많은 이익을 내는 프로젝트를 항상 선택해야 한다.

우리가 어떤 프로젝트를 감당할 수 있는지 어떻게 알 수 있을까? 우리는 `capital`에 따라 입력값을 정렬할 수 있고, 우리가 감당할 수 있는 가장 비싼 프로젝트의 인덱스를 저장하는 포인터 `i`를 사용할 수 있다. 프로젝트를 완료하고 `w`를 추가할 때마다, `i`를 전진시켜 `w`보다 더 많은 비용이 드는 프로젝트를 가리키도록 할 수 있다.

우리는 각 단계에서 감당할 수 있는 프로젝트를 알고 있지만, 최대는 어떻게 찾을까? 우리가 어떤 주어진 단계에서 최대만 신경 쓴다면, 이것은 힙에 완벽하다. `i`를 증가시키면서, 프로젝트의 이익을 최대 힙에 넣고, 각 단계에서 힙에서 팝하여 `w`에 값을 추가한다.

> 요약하자면, 각 단계에서 감당할 수 있는 가장 이익이 많은 프로젝트를 선택하라. 가장 이익이 많은 프로젝트를 추적하고 더 많은 자본을 얻을 때마다 프로젝트를 힙에 추가하기 위해 힙을 사용하라.
{: .prompt-general }

### [예제 3] 상세 내용

첫째, 모든 단계에서 사용할 수 있는 가장 이익이 많은 프로젝트를 선택하는 탐욕적 전략이 왜 옳은지 설명해보자. 물론, 가장 이익이 많은 프로젝트는 우리가 달성하려고 하는 가장 큰 돈을 줄 것이다. 하지만 다른 결과도 있다 - 돈을 더 얻으면, 더 많은 프로젝트가 우리에게 가능해질 수 있다.

두 프로젝트 중에서 선택해야 한다고 가정해보자, 하나는 이익이 `x`이고 다른 하나는 `y`로, 여기서 `x < y`이다. 우리가 `x`를 선택해야 할 상황이 있을까? `x`를 선택하면, 몇몇 새로운 프로젝트에 접근할 수 있을지 모른다.

하지만 `y > x`이기 때문에 `y`를 선택했다면 그와 동일한 새 프로젝트에 접근하지 못하는 상황은 없다. 사실, `y`를 선택하는 것은 `x`를 선택하는 것보다 더 많은 프로젝트를 열 수 있을 것이다. 우리는 어떤 주어진 단계에서도 가장 이익이 많은 프로젝트 외에 다른 프로젝트를 선택하는 의미가 없다는 것을 알 수 있다.

이것은 우리의 문제를 두 부분으로 나눈다:

1. 우리가 더 많은 돈을 벌면서 현재 사용할 수 있는 프로젝트 찾기
2. 각 단계에서 최대 이익의 프로젝트 찾기

첫 번째 부분에 대해서, 만약 우리가 프로젝트 비용에 따라 입력값을 정렬한다면, 우리는 더 많은 돈을 벌면서 입력값을 단순히 반복할 수 있다. 우리가 반복하면서, 만약 우리가 너무 비싼 프로젝트를 찾으면, 우리는 정렬했기 때문에 오른쪽의 모든 프로젝트도 너무 비싸다는 것을 안다. 두 번째 부분에 대해서, 우리는 최대 힙을 사용할 수 있다. 우리가 더 많은 프로젝트를 잠금해제함에 따라, 우리는 그 프로젝트들의 이익을 힙에 넣을 수 있다. 각 단계에서, 우리는 단순히 힙에서 팝하여 가장 이익이 많은 프로젝트를 찾을 수 있다.

```cpp
class Solution {
public:
    int findMaximizedCapital(int k, int w, vector<int>& profits, vector<int>& capital) {
        int n = int(profits.size());
        vector<pair<int, int>> projects;
        for (int i = 0; i < n; i++) {
            projects.push_back({capital[i], profits[i]});
        }
        
        sort(projects.begin(), projects.end());
        priority_queue<int> heap;
        int i = 0;
        
        for (int j = 0; j < k; j++) {
            while (i < n && projects[i].first <= w) {
                heap.push(projects[i].second);
                i++;
            }
            
            if (heap.size() == 0) {
                return w;
            }
            
            w += heap.top();
            heap.pop();
        }
        
        return w;
    }
};
```

이 알고리즘은 시간 복잡도가 $$O((k + n) \cdot \log{}n)$$이다. 여기서 n은 주어진 프로젝트의 수다. 힙의 최대 크기는 `n`이므로, 최악의 경우에 그 연산은 $$\log{}n$$이며, `k + n` 연산을 한다(`k`번의 팝 연산, `n`번의 푸시 연산). 처음에 정렬하는 것도 $$O(n \cdot \log{}n)$$의 비용이 들지만, 이것은 복잡도를 바꾸지 않는다. 공간 복잡도는 힙 때문에 $$O(n)$$이다.

---

## **예제 4: K 제거 후 고유한 정수의 최소 수**

> [문제 링크](https://leetcode.com/problems/least-number-of-unique-integers-after-k-removals/)
>
> 정수 배열 `arr`과 정수 `k`가 주어졌을 때, 정확히 `k` 요소를 제거한 후 고유한 정수의 최소 수를 찾으시오.
{: .prompt-general }

우리는 `k` 제거를 수행해야 한다 - 어떤 요소를 제거하는 것이 가장 좋을까? 우리는 요소를 **모두** 제거할 경우에만 정답을 개선할 수 있으므로, 각 단계에서 가장 낮은 빈도의 요소를 탐욕적으로 제거해야 한다.

각 요소의 빈도수를 찾기 위해 해시 맵을 사용할 수 있다. 그런 다음 빈도수에 따라 키를 정렬하고, 가장 빈도수가 낮은 요소부터 시작하여 키를 반복할 수 있다. 각 키에서, 빈도수가 `k` 이하인 경우 해당 키를 제거하고 `k`를 카운트만큼 감소시킬 수 있다. 제거할 것이 더 이상 없을 때까지 계속한다. 마지막에 남아있는 키의 수가 정답이다.

```cpp
class Solution {
public:
    int findLeastNumOfUniqueInts(vector<int>& arr, int k) {
        unordered_map<int, int> counts;
        for (int num: arr) {
            counts[num]++;
        }
        
        vector<int> ordered;
        for (auto [key, val]: counts) {
            ordered.push_back(val);
        }
        
        sort(ordered.begin(), ordered.end(), greater<int>());
        while (k > 0) {
            int val = ordered.back();
            if (val <= k) {
                k -= val;
                ordered.pop_back();
            } else {
                break;
            }
        }
        
        return ordered.size();
    }
};
```

---

## **예제 5: 구조 보트**

> [문제 링크](https://leetcode.com/problems/boats-to-save-people/)
>
> `people`이라는 배열이 주어지는데, `people[i]`는 $$i^{th}$$ 사람의 무게다. 보트는 두 사람을 수용할 수 있지만, 그들의 총 무게가 `limit` 이하일 때만 가능하다. 모든 사람을 수송하기 위해 필요한 보트의 최소 수는 몇 척인가? 참고: 어떤 사람도 `limit` 무게를 초과하지 않는다.
{: .prompt-general }

보트의 수를 최소화하고 싶으므로, 두 사람을 태우는 보트의 수를 최대화하고 싶다. 어떤 순간에 가장 무거운 사람이 `x`이고, 가장 가벼운 사람이 `y`라고 하자. 만약 `x + y > limit`이면, `x`는 아무와도 짝을 이룰 수 없고 자신의 보트를 사용해야 한다. 따라서 가장 무거운 사람마다 가장 가벼운 사람과 짝을 이루려고 탐욕적으로 시도해야 한다.

이 뒤에 있는 직관은 다음과 같다: 가장 가벼운 사람이 가장 무거운 사람과 짝을 이룰 수 있다면, 왜 가장 가벼운 사람과 짝을 이루기 위해 다른 사람을 선택하려 하는가? 모든 짝은 같은 값을 제공한다 - 정답을 1만큼 줄일 것이다. 가장 무거운 사람과 짝을 이루면, 미래에 더 많은 짝을 만들기 쉬워진다.

주어진 시간에 가장 가벼운 사람과 가장 무거운 사람을 추적하기 위해, 입력을 정렬하고 두 포인터를 사용한다. `i`가 가장 가벼운 사람을 가리키게 하고, `j`가 가장 무거운 사람을 가리키게 한다.

### [예제 5] 상세 내용

가장 가벼운 사람과 가장 무거운 사람을 각 단계에서 짝지으려는 이 탐욕적 전략이 최적인 이유를 설립해보자. `x`가 어떤 단계에서 가장 무거운 사람을 나타내고, `y`가 가장 가벼운 사람을 나타낸다고 하자.

두 가지 가능성이 있다:

1. `x + y > limit`. `y`가 이미 가장 가벼운 사람이기 때문에 `x`가 다른 사람과 함께 탈 수 있는 방법이 없다. 따라서 그들은 함께 앉아야 한다.
2. `x + y <= limit`. 이는 `y`가 `x`가 이미 가장 무거운 사람이기 때문에 누구와도 짝을 이룰 수 있다는 것을 의미한다.

보트의 효율성을 극대화하기 위해, `y`를 누구와도 짝지을 수 있다면, 가장 무거운 사람인 `x`와 짝을 이루어야 한다. 이렇게 하면 보트를 가장 잘 활용할 수 있으며, 더 이상 `x`에 대해 걱정할 필요가 없기 때문에 미래의 짝짓기를 더 쉽게 만든다.

다시 말하지만, 이것은 공식적인 증명은 아니지만, 면접에서 생각 과정을 설명할 수 있는 예시다. 여기서부터, 알고리즘은 정렬과 두 포인터를 사용하여 쉽게 구현할 수 있다.

```cpp
class Solution {
public:
    int numRescueBoats(vector<int>& people, int limit) {
        int ans = 0;
        int i = 0;
        int j = people.size() - 1;
        sort(people.begin(), people.end());
        
        while (i <= j) {
            if (people[i] + people[j] <= limit) {
                i++;
            }
            
            j--;
            ans++;
        }
        
        return ans;
    }
};
```

while 루프의 모든 반복에서 현재 가장 무거운 사람을 보트에 태운다(`j--`를 실행하는 이유). 만약 가장 가벼운 사람도 함께 갈 수 있다면 그들도 포함한다(`i++` 실행).

이미 본 두 포인터 구현과 마찬가지로, 알고리즘의 두 포인터 부분은 $$O(n)$$에서 실행된다. 여기서 $$n$$ = `people.length`이다. 그러나 이 알고리즘이 작동하려면 입력을 정렬해야 했으므로, 시간 복잡도는 $$O(n \cdot \log{}n)$$이 된다. 추가로 사용되는 공간은 정렬을 위한 것 뿐이며, 이전에 언급했듯이 사용하는 언어에 따라 달라진다.

예시에서 볼 수 있듯이, 대부분의 탐욕적 문제는 입력 정렬을 포함한다. 이전 장(힙)에서 살펴본 많은 문제들, 특히 탐욕적 문제들이 이러한 접근 방식을 사용했다고 주장할 수 있다. 대부분의 탐욕적 문제는 탐욕적 접근 방식이 효과적임을 파악하기 위해 일정한 논리적 추론을 필요로 하고, 그 후 구현은 비교적 간단하다. 다음 문제를 시도하기 전에 이 문제들을 시도해 보자.

---

## **보너스 문제**

- [1833. Maximum Ice Cream Bars](https://leetcode.com/problems/maximum-ice-cream-bars/)
- [409. Longest Palindrome](https://leetcode.com/problems/longest-palindrome/)
- [455. Assign Cookies](https://leetcode.com/problems/assign-cookies/)
- [1005. Maximize Sum Of Array After K Negations](https://leetcode.com/problems/maximize-sum-of-array-after-k-negations/)
- [2410. Maximum Matching of Players With Trainers](https://leetcode.com/problems/maximum-matching-of-players-with-trainers/)
- [1663. Smallest String With A Given Numeric Value](https://leetcode.com/problems/smallest-string-with-a-given-numeric-value/)
- [2486. Append Characters to String to Make Subsequence](https://leetcode.com/problems/append-characters-to-string-to-make-subsequence/)
- [2405. Optimal Partition of String](https://leetcode.com/problems/optimal-partition-of-string/)
- [11. Container With Most Water](https://leetcode.com/problems/container-with-most-water/)
- [2384. Largest Palindromic Number](https://leetcode.com/problems/largest-palindromic-number/)
- [2178. Maximum Split of Positive Even Integers](https://leetcode.com/problems/maximum-split-of-positive-even-integers/)
- [2139. Minimum Moves to Reach Target Score](https://leetcode.com/problems/minimum-moves-to-reach-target-score/)
- [2457. Minimum Addition to Make Integer Beautiful](https://leetcode.com/problems/minimum-addition-to-make-integer-beautiful/)
- [1024. Video Stitching](https://leetcode.com/problems/video-stitching/)
- [2136. Earliest Possible Day of Full Bloom](https://leetcode.com/problems/earliest-possible-day-of-full-bloom/)
- [1326. Minimum Number of Taps to Open to Water a Garden](https://leetcode.com/problems/minimum-number-of-taps-to-open-to-water-a-garden/)

---

출처: [Leetcode](https://leetcode.com/explore/interview/card/leetcodes-interview-crash-course-data-structures-and-algorithms/709/greedy/4647/)

<!--

{: .prompt-general }

-->