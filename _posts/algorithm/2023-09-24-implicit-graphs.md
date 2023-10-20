---
title: "묵시적 그래프"
categories: [Algorithm 연구소]
tags: [Algorithm]
date: 2023-09-24 01:30
math: true
img_path: /assets/img/algorithm/
---

---

![Title](algorithm_title.png)

---

## **개요**

처음 그래프를 살펴볼 때, 우리는 그래프의 일반적인 입력 형식(인접 리스트, 인접 행렬, 간선 배열, 행렬)에 대해 이야기했다. 지금까지 살펴본 문제들은 이러한 입력 형식을 따랐기 때문에 문제를 그래프로 모델링해야 한다는 것을 알아차리기 쉬웠다. 어떤 문제들은 우리가 그래프를 다루고 있다고 명시적으로 말해 주었고, 일부는 "도로로 연결된 도시들"과 같은 스토리를 가지고 있었다. 어쨌든, 입력 형식은 보통 단서였다.

때로는 그래프가 더 미묘할 수 있다. 입력이 우리가 얘기한 형식 중 어느 것과도 다를 수 있다. 그래프는 원소(노드)들의 추상적인 집합이며, 이들은 일부 추상적인 관계(간선)에 의해 연결된다는 것을 기억하라. 문제가 상태 간의 전환을 포함한다면, 상태를 노드로, 전환 기준을 간선으로 생각해 볼 수 있다. 또한 문제가 최단 경로나 가장 적은 연산 등을 원한다면, 이는 BFS에 대한 훌륭한 후보이다. 몇 가지 예를 살펴보자.

---

> **예제 1**: [자물쇠 열기](https://leetcode.com/problems/open-the-lock/)
>
> 4개의 원형 바퀴가 달린 자물쇠가 있다. 각 바퀴는 `0`부터 `9`까지의 숫자를 가지고 있다. 바퀴는 회전하며 둘러싸고 있다 - 그래서 `0`은 `9`로 바뀔 수 있고, 9는 0으로 바뀔 수 있다. 초기에 자물쇠는 `"0000"`을 표시한다. 한 번의 이동은 바퀴를 한 칸 돌리는 것으로 구성된다. `deadends`라는 차단된 코드 배열이 주어지는데 - 자물쇠가 이 코드 중 어느 것을 읽으면 더 이상 돌릴 수 없다. 자물쇠가 목표를 표시하도록 만드는 최소 이동 횟수를 반환하라.
{: .prompt-general }

이 문제에서, 각 자물쇠 상태를 노드로 간주할 수 있다. 간선은 한 위치에서 `1`만큼 차이나는 모든 노드다. 예를 들어, `"5231"`과 `"5331"`은 이웃이다. 여기서 우리는 단순히 `"0000"`에서 BFS를 수행할 수 있는데, `deadends`에 있는 어떤 노드도 방문할 수 없다는 조건 하나만 있으면 된다. $$O(1)$$ 확인을 위해, BFS를 시작하기 전에 `deadends`를 세트로 변환하자.

노드의 이웃을 찾으려면, 4개의 슬롯 각각을 반복하고 각 슬롯에서 슬롯을 `1`씩 증가시키고 감소시킬 수 있다. 둘러싸는 경우를 처리하려면 모듈로 연산자를 사용할 수 있다 - `decrement(x) = (x - 1) % 10` 이고 `increment(x) = (x + 1) % 10`. 이는 `decrement(0) = 9` 이고 `increment(9) = 0` 때문에 효과적이다.

> 조금 더 깔끔하게 하기 위해, `deadends`의 모든 차단된 코드를 BFS를 시작하기 전에 `seen`에 넣을 수 있고, 이웃이 `deadends`에 있는지 확인하기 위한 추가적인 `if` 확인을 추가하는 대신에 할 수 있다.
{: .prompt-general }

```cpp
class Solution {
public:
    int openLock(vector<string>& deadends, string target) {
        if (find(deadends.begin(), deadends.end(), "0000") != deadends.end()) {
            return -1;
        }
        
        queue<pair<string, int>> queue;
        queue.push(pair("0000", 0));
        unordered_set<string> seen(deadends.begin(), deadends.end());
        
        while (!queue.empty()) {
            auto [node, steps] = queue.front();
            queue.pop();
            
            if (node == target) {
                return steps;
            }
            
            for (string neighbor: neighbors(node)) {
                if (seen.find(neighbor) == seen.end()) {
                    seen.insert(neighbor);
                    queue.push(pair(neighbor, steps + 1));
                }
            }
        }
        
        return -1;
    }
    
    vector<string> neighbors(string node) {
        vector<string> ans;
        for (int i = 0; i < 4; i++) {
            int num = node[i];
            for (int change: {-1, 1}) {
                int x = (num - '0' + change + 10) % 10 + '0';
                string neighbor = node;
                neighbor[i] = x;
                ans.push_back(neighbor);
            }
        }
        
        return ans;
    }
};
```

이 알고리즘의 기술적인 시간 복잡도는 $$O(d)$$로, 여기서 `d`는 `deadends`를 세트로 변환하는 데 필요한 `deadends`의 길이다. 이 문제에서 나머지 모든 것은 상수(4개의 슬롯, 10개의 숫자)이기 때문이다. 하지만 자물쇠에 슬롯의 수가 가변적이라면, 예를 들어 `n`이라면, 이것은 시간 복잡도를 $$O(10^n \cdot n^2 + d)$$로 변경한다. 각 슬롯에 10개의 옵션이 있기 때문에 $$10^n$$개의 다른 상태가 있다. 각 상태에서, 우리는 $$O(n^2)$$의 작업을 수행한다. 왜냐하면 불변하는 문자열에 대해서 $$O(n)$$인 문자열 연결을 수행하면서 `n`개의 슬롯을 순회하기 때문이다.

---

> **예제 2**: [나눗셈 평가](https://leetcode.com/problems/evaluate-division/)
>
> 배열 `equations`와 동일한 길이의 숫자 배열 `values`가 주어진다. `equations[i] = [x, y]`는 `x / y = values[i]`를 나타낸다. 또한 `queries` 배열이 주어지는데 `queries[i] = [a, b]`는 몫 `a / b`를 나타낸다. `answer` 배열을 반환하는데 `answer[i]`는 $$i^{th}$$ 쿼리에 대한 답이거나, 결정될 수 없다면 `-1`이다.
{: .prompt-general }

우리는 $$\frac{y}{x} = val$$인 방정식을 몇 개 가지고 있다. 상상하기 어려울 수 있지만, 이 방정식들만으로도 그래프를 설명할 수 있다!

지금까지 살펴본 모든 그래프 문제에서, 그래프들은 **가중치가 없었다**. **가중치**는 간선과 관련된 값이다. 대부분의 면접에서 가중치가 있는 그래프는 범위를 벗어나며 고급 알고리즘을 요구하기 때문에 아직 그들에 대해 이야기하지 않았다. 이 문제에서, 각 변수 `x`와 `y`를 노드로 취급할 수 있다. 간선은 `equations`에서 제공되며, 가중치는 `values`에서 제공된다.

`a, b, c`라는 3개의 변수가 있다고 가정해보자. 우리는 `a / b = 5` 그리고 `b / c = 2`라는 것을 알고 있다. `a / c`는 얼마인가? 몇 가지 값을 대입해보자. 예를 들어 `a = 10, b = 2, c = 1`이라고 하자. 그러면 방정식이 성립하고, `a / c = 10`임을 알 수 있다. 분석 대신에 그래프 순회를 통해 이것을 어떻게 달성할 수 있을까?

$$\frac{a}{b} = 5$$는 `a`가 `b`보다 `5`배 더 크다는 것을 알려준다. 만약 `a`와 `b`가 그래프의 노드라면, 간선 `a -> b`를 사용하면 `5`배의 곱셈이 이루어진다. `b / c = 2`는 `b`가 `c`보다 `2`배 더 크다는 것을 말한다. 만약 간선 `b -> c`를 사용한다면 `2`배의 곱셈이 이루어진다. 따라서, `a`에서 `c`로의 순회는 `5 * 2 = 10`의 곱셈을 제공한다.

입력에서 주어진 관계를 포함하는 다른 해시 맵에 매핑되는 각 노드에 대한 해시 맵을 사용하여 `graph`를 생성할 수 있다. 그러므로 위의 예에서 `a`는 `b: 5`를 매핑하는 해시 맵에 매핑될 것이다. 그런 다음, 각 쿼리에 대해 순회(BFS 또는 DFS, 상관없음)를 할 수 있다 - 분자에서 시작하여 현재 비율/배율(초기에는 `1`)을 각 노드와 연관시키면서 분모를 찾는다. 각 이웃 순회에서, 우리는 현재 비율을 이웃 사이의 비율로 곱한다.

한 가지 더: $$\frac{y}{x} = val$$이면, $$\frac{x}{y} = \frac{1}{val}$$도 함축하므로 우리의 그래프를 구축할 때 이것을 포함해야 한다(간선은 무방향이다).

```cpp
class Solution {
public:
    unordered_map<string, unordered_map<string, double>> graph;
    
    vector<double> calcEquation(vector<vector<string>>& equations, vector<double>& values, vector<vector<string>>& queries) {
        for (int i = 0; i < equations.size(); i++) {
            string numerator = equations[i][0], denominator = equations[i][1];
            double val = values[i];
            graph[numerator][denominator] = val;
            graph[denominator][numerator] = 1 / val;
        }
        
        vector<double> ans;
        for (int i = 0; i < queries.size(); i++) {
            ans.push_back(answerQuery(queries[i][0], queries[i][1]));
        }
        
        return ans;
    }

    double answerQuery(string start, string end) {
        // 이 노드에 아무런 정보도 없을 때
        if (graph.find(start) == graph.end()) {
            return -1;
        }
        
        unordered_set<string> seen({start});
        stack<pair<string, double>> stack({pair(start, 1)});
        
        while (!stack.empty()) {
            auto [node, ratio] = stack.top();
            stack.pop();
            if (node == end) {
                return ratio;
            }

            for (auto [neighbor, val]: graph[node]) {
                if (seen.find(neighbor) == seen.end()) {
                    seen.insert(neighbor);
                    stack.push(pair(neighbor, ratio * val));
                }
            }
        }
        
        return -1;
    }
};
```

변수의 수가 `n`이고, `equations`에서 주어진 것이며(최악의 경우에 `equations.length`와 선형적), queries의 길이가 q라면, 시간 복잡도는 $$O(q \cdot (n+e))$$이다. `answerQuery`에 대한 각 호출은 우리가 구축한 그래프에서의 순회인데, 이는 우리가 알다시피 노드 수와 간선 수인 `n + e`를 비용으로 한다. 우리는 `q`번의 순회를 수행한다. 만약 출력에 사용되는 공간을 추가 공간으로 계산하지 않는다면, 공간 복잡도는 `graph`를 구축하고, `seen` 및 재귀 호출 스택에 대해 $$O(n + e)$$이다.

---

## **보너스 문제**

### 이진 트리

- [100. Same Tree](https://leetcode.com/problems/same-tree/)
- [872. Leaf-Similar Trees](https://leetcode.com/problems/leaf-similar-trees/)
- [226. Invert Binary Tree](https://leetcode.com/problems/invert-binary-tree/)
- [101. Symmetric Tree](https://leetcode.com/problems/symmetric-tree/)
- [113. Path Sum II](https://leetcode.com/problems/path-sum-ii/)
- [1325. Delete Leaves With a Given Value](https://leetcode.com/problems/delete-leaves-with-a-given-value/)
- [437. Path Sum III](https://leetcode.com/problems/path-sum-iii/)
- [1372. Longest ZigZag Path in a Binary Tree](https://leetcode.com/problems/longest-zigzag-path-in-a-binary-tree/)

### 이진 트리: BFS

- [102. Binary Tree Level Order Traversal](https://leetcode.com/problems/binary-tree-level-order-traversal/)
- [1161. Maximum Level Sum of a Binary Tree](https://leetcode.com/problems/maximum-level-sum-of-a-binary-tree/)
- [637. Average of Levels in Binary Tree](https://leetcode.com/problems/average-of-levels-in-binary-tree/)
- [1609. Even Odd Tree](https://leetcode.com/problems/even-odd-tree/)

### 이진 탐색 트리

- [700. Search in a Binary Search Tree](https://leetcode.com/problems/search-in-a-binary-search-tree/)
- [1305. All Elements in Two Binary Search Trees](https://leetcode.com/problems/all-elements-in-two-binary-search-trees/)
- [235. Lowest Common Ancestor of a Binary Search Tree](https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-search-tree/)
- [450. Delete Node in a BST](https://leetcode.com/problems/delete-node-in-a-bst/)

### 그래프

- [997. Find the Town Judge](https://leetcode.com/problems/find-the-town-judge/)
- [1615. Maximal Network Rank](https://leetcode.com/problems/maximal-network-rank/)
- [463. Island Perimeter](https://leetcode.com/problems/island-perimeter/)
- [1020. Number of Enclaves](https://leetcode.com/problems/number-of-enclaves/)
- [1376. Time Needed to Inform All Employees](https://leetcode.com/problems/time-needed-to-inform-all-employees/)
- [2192. All Ancestors of a Node in a Directed Acyclic Graph](https://leetcode.com/problems/all-ancestors-of-a-node-in-a-directed-acyclic-graph/)
- [990. Satisfiability of Equality Equations](https://leetcode.com/problems/satisfiability-of-equality-equations/)
- [886. Possible Bipartition](https://leetcode.com/problems/possible-bipartition/)
- [994. Rotting Oranges](https://leetcode.com/problems/rotting-oranges/)

---

출처: [Leetcode](https://leetcode.com/explore/interview/card/leetcodes-interview-crash-course-data-structures-and-algorithms/707/traversals-trees-graphs/4635/)

<!--

{: .prompt-general }

-->