---
title: "이진 트리-BFS"
categories: [Algorithm 연구소]
tags: [Algorithm]
date: 2023-09-20 01:30
math: true
img_path: /assets/img/algorithm/
---

---

![Title](algorithm_title.png)

---

## **개요**

DFS(깊이 우선 탐색)에서는 깊이를 우선시하며, BFS(너비 우선 탐색)에서는 너비를 우선시한다. 노드의 깊이는 루트 노드로부터의 거리를 의미한다. DFS에서는 현재 노드에서 가능한 한 깊숙이 파고들며, 리프 노드에 도달할 때까지 깊이를 증가시키려고 한다. 큰 트리에서 DFS를 수행할 때, 방문할 노드의 깊이는 `0, 1, 2, 3, 4, 5, 6, ....` 등으로 나타날 수 있다.

BFS는 현재 깊이에 있는 **모든 노드를 방문한 후에야 다음 깊이의 노드로 넘어간다**. 따라서 [완전 이진 트리](https://en.wikipedia.org/wiki/Binary_tree#Types_of_binary_trees)에서 BFS를 수행하면, 탐색하는 노드의 깊이는 `0, 1, 1, 2, 2, 2, 2, 2, 3, 3, ....` 등과 같이 나타난다.

> "완전 이진 트리"란, 모든 레벨이 노드로 가득 차 있고 마지막 레벨에서는 노드가 왼쪽부터 채워져 있는 트리를 말한다.
{: .prompt-general }

트리의 각 깊이를 "레벨"로 간주한다면, 트리를 하나의 건물처럼 생각할 수 있으며 이 건물에서 루트는 꼭대기 층이고, 가장자리는 아래층으로 내려가는 계단처럼 볼 수 있다.

![Bfs](bfs.png)

위의 트리 구조에서 BFS가 수행되면, 노드는 해당 값에 따라 방문된다. 깊이 `d`의 모든 노드를 방문한 후, 깊이 `d + 1`의 노드를 방문한다.

DFS는 스택을 사용하여 구현할 수 있으며 (재귀 호출은 내부적으로 스택을 사용), BFS는 큐를 사용한 반복적인 방법으로 구현한다. BFS를 재귀적으로 구현할 수는 있지만, 복잡하며 특별한 이점을 제공하지 않기 때문에, 일반적으로 BFS는 반복적인 방법을 통해 구현된다.

---

## **BFS와 DFS는 언제 사용해야 할까?**

이전에 다룬 내용에서는 전위, 중위, 후위 순회 같은 DFS의 어떤 방법을 사용하든, 중요한 것은 모든 노드를 방문하는 것이었다고 언급했다. 이러한 유형의 문제에서는 노드를 '방문'할 때마다 필요한 정보를 저장하므로, 방문 순서가 문제에 크게 영향을 미치지 않는다. 그래서 BFS를 사용해도 무방하다.

이진 트리 알고리즘 문제에서 DFS가 BFS보다 더 우수한 경우는 드물다. 일반적으로 DFS 구현이 코드가 더 간결하므로 선호되며, 재귀를 이용한 구현이 더 간단하기 때문에 BFS나 DFS가 중요하지 않은 경우 대부분의 개발자는 DFS를 선택하는 경향이 있다.

반면에, BFS가 DFS보다 알고리즘적으로 훨씬 더 적합한 문제도 많이 있다. 특히 노드를 레벨별로 처리해야 하는 문제에서는 BFS가 더 적합하다. 이에 대한 예제와 연습 문제는 다음에 설명할 예정이다.

면접에서는 BFS와 DFS의 단점 등을 묻는 질문을 받을 수도 있다. DFS의 가장 큰 단점은 찾고자 하는 값이 트리의 끝 쪽에 있을 때, 검색 시간이 오래 걸릴 수 있다는 것이다. 예를 들어, **큰** 트리에서 루트의 오른쪽 자식에 있는 값을 찾고자 하는데, DFS가 왼쪽을 우선 탐색한다면 왼쪽 하위 트리 **전체**를 탐색해야 하므로 수많은 연산이 필요하다. 반면, 해당 노드는 루트 바로 아래에 있어 BFS를 사용하면 단 한 번의 연산으로 찾을 수 있다. BFS의 단점은 찾고자 하는 노드가 트리의 아래쪽에 있을 때, 모든 레벨을 탐색해야 하므로 시간이 오래 걸릴 수 있다는 점이다.

공간 복잡도 측면에서 DFS는 트리의 높이에 비례하는 공간을 사용하고, BFS는 트리의 가장 넓은 레벨에 비례하는 공간을 사용한다. 상황에 따라 DFS가 더 적은 공간을 차지할 수도 있고, BFS가 더 효율적일 수도 있다.

예를 들어, [완전 이진 트리](https://en.wikipedia.org/wiki/Binary_tree#Types_of_binary_trees)에서 DFS는 $$O(\log{}n)$$의 공간을 사용하지만, BFS는 $$O(n)$$의 공간을 필요로 한다. 완전 이진 트리의 마지막 레벨에는 $$\frac{n}{2}$$ 개의 노드가 있지만, 트리의 높이는 $$\log{}n$$이다.

그러나 편향된 트리와 같은 특수한 경우에는 BFS의 공간 복잡도가 $$O(1)$$이 될 수 있고, DFS는 $$O(n)$$이 될 수 있다(편향된 트리는 일반적이지 않으며, 대부분의 경우 완전 또는 균형 잡힌 트리를 다루게 된다).

---

## **BFS 코드 구현**

BFS의 코드 구현은 DFS와 마찬가지로 여러 문제에서 매우 유사하다. 다음은 BFS를 이용하여 트리의 노드 값을 출력하는 일반적인 형식의 예시이다.

```cpp
void printAllNodes(TreeNode* root) {
    queue<TreeNode*> queue;
    queue.push(root);
    
    while (!queue.empty()) {
        int nodesInCurrentLevel = queue.size();
        // 현재 레벨에 대한 로직 수행

        for (int i = 0; i < nodesInCurrentLevel; i++) {
            TreeNode* node = queue.front();
            queue.pop();

            // 현재 노드에 대한 로직 수행
            cout << node->val << endl;
            
            // 다음 레벨의 노드를 큐에 넣기
            if (node->left) {
                queue.push(node->left);
            }
            
            if (node->right) {
                queue.push(node->right);
            }
        }
    }
}
```

> JavaScript 사용자들은 JavaScript가 내장된 효율적인 큐를 제공하지 않는다는 점을 유의해야 한다. 그러나 `nextQueue`라는 두 번째 배열을 사용하여 이 문제를 해결하고 효율적인 BFS를 구현할 수 있다.
{: .prompt-general }

while 루프의 각 반복이 시작될 때 큐에는 현재 레벨의 모든 노드가 있어야 한다. 처음 시작할 때 이는 루트 노드 뿐이다.

그 다음 for 루프를 사용하여 현재 레벨의 노드들을 반복 처리한다. 현재 레벨의 노드 수를 `nodesInCurrentLevel`에 저장하여 for 루프가 다른 노드로 넘어가지 않도록 한다. 이 for 루프는 현재 레벨의 각 노드를 방문하고, 자식 노드들(다음 레벨)을 큐에 추가한다.

큐는 '선입선출'의 원칙을 따르므로, for 루프가 종료될 때, 큐에는 다음 레벨의 모든 노드가 들어 있게 된다. 이후 다음 while 루프 반복으로 넘어가며 이 과정이 계속 반복된다.

효율적인 큐를 사용할 경우, 큐에서의 추가 및 제거 작업은 $$O(1)$$의 시간 복잡도를 가지므로 BFS의 시간 복잡도는 DFS와 동일하다. 중요한 것은 각 노드를 한 번만 방문한다는 것이며, 따라서 시간 복잡도는 $$O(n \cdot k)$$이 된다. 여기서 $$n$$은 총 노드 수이고, $$k$$는 각 노드에서 수행하는 작업의 양을 의미한다. 이 $$k$$는 일반적으로 $$O(1)$$이다. 이제 몇 가지 예제 문제를 통해 이해를 돕도록 하겠다.

---

> **예제 1**: [이진 트리 오른쪽 사이드 뷰](https://leetcode.com/problems/binary-tree-right-side-view/)
>
> 이진 트리의 `root`가 주어졌을 때, 그 트리를 오른쪽 측면에서 바라봤을 때 보이는 노드의 값들을 상단부터 하단 순서로 반환한다.
{: .prompt-general }

이 문제는 기본적으로 각 레벨에서 가장 오른쪽에 있는 노드에 무엇이 있는지 묻는 것이다. 너비 우선 탐색(BFS) 알고리즘을 사용하면, 각 반복마다 현재 층의 모든 노드들을 방문하는 동안 그 층의 가장 오른쪽 노드를 확인할 수 있다. BFS는 오른쪽 자식 노드를 왼쪽 자식 노드보다 먼저 방문하기 때문에, 각 반복에서 마지막에 방문하는 노드는 그 층의 가장 오른쪽 노드가 된다.

```cpp
class Solution {
public:
    vector<int> rightSideView(TreeNode* root) {
        if (root == nullptr) {
            return vector<int>{};
        }
        
        vector<int> ans;
        queue<TreeNode*> queue;
        queue.push(root);
        
        while (!queue.empty()) {
            int currentLength = queue.size();
            ans.push_back(queue.back()->val);
            
            for (int i = 0; i < currentLength; i++) {
                TreeNode* node = queue.front();
                queue.pop();
                
                if (node->left) {
                    queue.push(node->left);
                }
                
                if (node->right) {
                    queue.push(node->right);
                }
            }
        }
        
        return ans;
    }
};
```

이 알고리즘은 각 노드를 단 한 번씩 방문하고 각 노드에서 상수 시간에 처리를 수행하기 때문에 시간 복잡도는 $$O(n)$$이다. 여기서 n은 트리의 노드 수이다. 또한 큐가 트리의 모든 노드를 담을 수 있기 때문에 공간 복잡도 역시 $$O(n)$$이다.

---

> **예제 2**: [각 트리 행에서 가장 큰 값 찾기](https://leetcode.com/problems/find-largest-value-in-each-tree-row/)
>
> 이진 트리의 `root`가 주어지면, 트리의 각 행에서 가장 큰 값을 담은 배열을 반환한다.
{: .prompt-general }

이진 트리에서 "행"이란 트리의 "레벨"과 같은 의미로 사용된다. while 루프 내의 각 반복은 트리의 한 레벨을 처리함을 의미한다. 각 레벨에서 가장 큰 값을 찾기 위해, `currMax`라는 정수 변수를 사용한다. 이전 예제와 이번 예제의 코드가 얼마나 유사한지 주목할 필요가 있다.

```cpp
class Solution {
public:
    vector<int> largestValues(TreeNode* root) {
        if (root == nullptr) {
            return vector<int>{};
        }
        
        vector<int> ans;
        queue<TreeNode*> queue;
        queue.push(root);
        
        while (!queue.empty()) {
            int currentLength = queue.size();
            int currMax = INT_MIN;
            
            for (int i = 0; i < currentLength; i++) {
                TreeNode* node = queue.front();
                currMax = max(currMax, node->val);
                queue.pop();
                
                if (node->left) {
                    queue.push(node->left);
                }
                
                if (node ->right) {
                    queue.push(node->right);
                }
            }
            
            ans.push_back(currMax);
        }
        
        return ans;
    }
};
```

시간 복잡도와 공간 복잡도는 모두 $$O(n)$$이다. 이는 트리의 모든 노드를 한 번씩 방문하기 때문이며, 큐에는 한 번에 모든 노드가 들어갈 수 있는 최대 노드 수만큼의 공간이 필요하기 때문이다.

이진 트리에서 BFS와 DFS의 선택은 대부분 상황에 따라 달라지지만, 특정 경우에는 BFS가 더 유리한 상황이 있음을 이 예제를 통해 알 수 있다. 특히 그래프 문제에서 BFS는 매우 강력하게 작용할 수 있는데, 이에 대한 예를 몇 개 더 접하게 될 것이다. 이 문제들을 해결해보면 BFS에 대한 이해도가 더욱 깊어질 것이다.

---

출처: [Leetcode](https://leetcode.com/explore/interview/card/leetcodes-interview-crash-course-data-structures-and-algorithms/707/traversals-trees-graphs/4619/)

<!--

{: .prompt-general }

-->