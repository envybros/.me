---
title: "이진 탐색 트리"
categories: [Algorithm 연구소]
tags: [Algorithm]
date: 2023-09-21 01:30
math: true
img_path: /assets/img/algorithm/
---

---

![Title](algorithm_title.png)

---

## **개요**

이진 탐색 트리(BST)는 이진 트리의 한 유형으로, 각 노드에 대해 왼쪽 하위 트리의 모든 값은 노드의 값보다 작고 오른쪽 하위 트리의 모든 값은 노드의 값보다 크다는 특징을 가지고 있다.

다음은 BST의 예시를 보여주는 그림이다:

![BST](binary_search_tree.png)

이진 탐색 트리를 활용하면, **이진 탐색** 원리를 적용하여 탐색, 추가, 제거 등의 작업을 평균적으로 $$O(\log{}n)$$ 시간 내에 수행할 수 있으며, 여기서 $$n$$은 트리의 노드 수를 의미한다.

예를 들어, 위의 트리에서 값 `20`의 존재 여부를 확인하려고 한다면, 루트에서 시작하여 현재 노드의 값과 찾고자 하는 값과 비교한다. 처음에는 `23`이 `20`보다 크기 때문에, `23`보다 작은 값들이 존재할 왼쪽 하위 트리로 진행하게 된다. 다음에는 `20`이 `8`보다 크므로, 이번에는 오른쪽 하위 트리로 방향을 전환한다. 이와 같은 과정을 거쳐 최종적으로 `20`을 찾아낼 수 있다.

이 과정의 평균 시간 복잡도는 $$O(\log{}n)$$이지만, 트리가 한 쪽으로 치우쳐져 완전히 비균형적인 상태인 경우(예: 모든 노드가 오직 한 쪽 방향의 자식만을 가진 경우) 최악의 시간 복잡도는 $$O(n)$$이 될 수 있다.

> 한편, BST를 이용한 특정한 깊이 우선 탐색(DFS) 방법은, 노드를 방문하는 순서에 따라 자동적으로 노드의 값을 정렬된 상태로 확인할 수 있게 해준다. 이 방식은 트리의 왼쪽 하위 트리를 우선적으로 방문한 다음, 오른쪽 하위 트리를 방문하게 되므로, 노드들이 저장된 값을 오름차순으로 점검하는 효과를 가진다.
{: .prompt-general }

---

> **예제 1**: [BST 범위 합계](https://leetcode.com/problems/range-sum-of-bst/)
>
> **이진 탐색 트리**의 `root` 노드와 `low`과 `high`이라는 두 정수가 주어졌을 때, [`low`, `high`] 범위에 속하는 모든 노드의 값을 합산하여 반환하는 문제이다.
{: .prompt-general }

가장 간단한 방법은 모든 노드를 방문하는 일반적인 BFS나 DFS 알고리즘을 사용하고, 그 과정에서 값이 `low`보다 낮거나 `high`보다 높은 노드는 합산에서 제외하는 것이다. 그러나 이진 탐색 트리의 속성을 이용하면 더 효율적인 알고리즘을 구현할 수 있다. BST에서 각 노드는 왼쪽 자식 노드보다 큰 값, 오른쪽 자식 노드보다 작은 값을 가진다. 그러므로 현재 노드의 값이 `low`보다 작다면, 왼쪽 자식 노드들은 검사할 필요가 없다. 또한, 현재 노드의 값이 `high`보다 크다면 오른쪽 자식 노드들을 검사할 필요가 없다. 이러한 최적화를 통해 많은 계산량을 줄일 수 있다.

```cpp
class Solution {
public:
    int rangeSumBST(TreeNode* root, int low, int high) {
        if (root == nullptr) {
            return 0;
        }
        
        int ans = 0;
        if (low <= root->val && root->val <= high) {
            ans += root->val;
        }
        if (low <= root->val) {
            ans += rangeSumBST(root->left, low, high);
        }
        if (root->val < high) {
            ans += rangeSumBST(root->right, low, high);
        }
        
        return ans;
    }
};
```

또한, 반복적인 방법으로 구현된 버전도 있다:

```cpp
class Solution {
public:
    int rangeSumBST(TreeNode* root, int low, int high) {
        stack<TreeNode*> stack;
        stack.push(root);
        int ans = 0;
        
        while (!stack.empty()) {
            TreeNode* node = stack.top();
            stack.pop();
            if (low <= node->val && node->val <= high) {
                ans += node->val;
            }
            if (node->left != nullptr && low <= node->val) {
                stack.push(node->left);
            }
            if (node->right != nullptr && node->val < high) {
                stack.push(node->right);
            }
        }
        
        return ans;
    }
};
```

트리의 모든 노드가 `low`과 `high`의 범위에 있을 때, 시간 복잡도는 여전히 $$O(n)$$이지만, 일반적으로 이 방법은 모든 노드를 단순히 순회하는 것보다 효율적이다. 예를 들어, 백만 개의 노드를 가진 트리가 있고, 루트의 값이 `high` 값보다 큰 경우, 오른쪽 하위 트리의 노드들은 모두 루트 값보다 크므로, 루트 기준으로 오른쪽 노드 방문을 즉시 절반 이상(`500,000`) 줄일 수 있다.

공간 복잡도는 스택 또는 재귀 호출에 따라 $$O(n)$$이 될 수 있다.

---

> **예제 2**: [BST의 최소 절대 차이](https://leetcode.com/problems/minimum-absolute-difference-in-bst/)
>
> BST의 `root`가 주어지면 트리에서 서로 다른 두 노드의 값 사이의 최소 절대 차이를 반환한다.
{: .prompt-general }

한 가지 접근 방식은 트리를 살펴보고 모든 값을 배열에 넣은 다음, 배열의 모든 쌍을 반복하여 최소 차이를 찾는 것이다. 이 방법은 $$O(n^2)$$의 시간 복잡도를 가진다. 더 나은 접근 방식은 배열을 정렬한 다음 인접한 요소 사이를 반복하는 것이다. 정렬된 배열에서 가장 작은 차이는 인접한 요소 사이에 있을 가능성이 높으므로, 이 방법은 시간 복잡도를 $$O(n \cdot \log{}n)$$으로 줄일 수 있다. 하지만 더 좋은 방법이 있을까?

이전에 간략하게 언급했듯이, BST에서 중위 순회를 수행하면 노드를 정렬된 순서로 방문할 수 있다. 따라서 중위 순회를 하는 것만으로도 정렬 과정 없이 노드를 정렬된 상태로 볼 수 있으며, 전체 시간 복잡도는 $$O(n)$$으로 줄어든다.

dfs 함수를 사용하여 `values` 배열에 값을 넣는다. 순서대로 순회하기 위해 먼저 왼쪽 자식을 방문한 다음 현재 노드의 값을 `values` 배열에 추가하고, 이후 오른쪽 자식을 방문한다. 이 방법을 사용하면 값이 정렬된 순서로 배열에 추가된다.

```cpp
class Solution {
public:
    int getMinimumDifference(TreeNode* root) {
        vector<int> values;
        dfs(root, values);
        int ans = INT_MAX;
        for (int i = 1; i < values.size(); i++) {
            ans = min(ans, values[i] - values[i - 1]);
        }
        
        return ans;
    }
    
    void dfs(TreeNode* node, vector<int>& values) {
        if (node == nullptr) {
            return;
        }
        
        dfs(node->left, values);
        values.push_back(node->val);
        dfs(node->right, values);
    }
};
```

이전에도 언급했듯이 반복적인 DFS는 전위 순회는 비교적 쉽지만, 중위나 후위 순회는 더 어렵다. 다음은 중위 순회를 사용하는 반복적인 해결 방법이다:

```cpp
class Solution {
public:
    int getMinimumDifference(TreeNode* root) {
        vector<int> values = iterativeInorder(root);
        int ans = INT_MAX;
        for (int i = 1; i < values.size(); i++) {
            ans = min(ans, values[i] - values[i - 1]);
        }

        return ans;
    }
    
    vector<int> iterativeInorder(TreeNode* root) {
        stack<TreeNode*> stack;
        vector<int> values;
        TreeNode* curr = root;
        
        while (!stack.empty() || curr != nullptr) {
            if (curr != nullptr) {
                stack.push(curr);
                curr = curr->left;
            } else {
                curr = stack.top();
                stack.pop();
                values.push_back(curr->val);
                curr = curr->right;
            }
        }

        return values;
    }
};
```

위에서 볼 수 있듯이, 반복적인 중위 순회는 재귀적인 방법보다 구현하기가 훨씬 더 복잡하다. 따라서 가능한 경우에는 재귀를 사용하는 것이 좋다. 면접에서는 두 가지 접근 방식을 모두 설명할 수 있도록 준비하는 것이 좋지만, 반복적인 중위 순회는 일반적인 경우에 비해 복잡할 수 있다.

이 접근 방식의 시간 복잡도와 공간 복잡도는 모두 $$O(n)$$이다. BST의 속성을 이용하면 정렬된 순서로 값을 얻을 수 있기 때문에, 선형 시간 안에 작업을 완료할 수 있다.

---

> **예제 3**: [이진 탐색 트리 유효성 검사](https://leetcode.com/problems/validate-binary-search-tree/)
>
> 이진 트리의 `root`가 주어졌을 때, 유효한 BST인지 확인한다.
{: .prompt-general }

재귀를 사용하여 `node`를 받고 해당 `node`에 연결된 트리가 BST인 경우 true를 반환하는 함수 `dfs`를 만들 수 있다. 먼저, 노드 외에 어떤 인수를 전달해야 할까? BST에서 루트 노드는 어떤 노드의 자식이 아니므로 어떤 값이라도 될 수 있지만, 왼쪽 하위 트리의 모든 노드는 루트 노드보다 작아야 하고 오른쪽 하위 트리의 모든 노드는 루트 노드보다 커야 한다. 이를 보장하기 위해 두 개의 정수 인자 `small`과 `large`을 사용하고 `small < node.val < large`이 유지되도록 할 수 있다.

허용되는 값의 범위로 `(small, large)`를 정의하는 경우, BST 속성을 유지하기 위해 어떻게 업데이트해야 할까? 각 `node`에서 왼쪽 하위 트리 노드는 `node.val`보다 작아야 하므로 `large = node.val`을 업데이트할 수 있다. 오른쪽 하위 트리 노드는 `node.val`보다 커야 하므로 `small = node.val`을 업데이트할 수 있다. 루트 노드의 경우 `small = -infinity`, `large = infinity` 초기화할 수 있다. 루트는 부모가 없으므로 어떤 값이라도 될 수 있다.

"BST에서 특정 노드의 데이터가 `val`이라고 가정해 보자. **주어진 노드**에서 왼쪽 하위 트리의 모든 데이터는 `val`보다 작고, 오른쪽 하위 트리의 모든 데이터는 `val`보다 크다." "주어진 노드에서"라는 표현 때문에, 이는 BST에서 **모든 하위 트리**도 BST라는 의미이기도 하다. 따라서 입력으로 `node`가 주어졌을 때 `node.left`와 `node.right`도 BST인지 확인해야 한다.

> 종료 조건은 무엇일까? 빈 트리(노드가 없는 트리)는 엄밀히 말해 BST이다. 따라서 현재 노드가 `null`일 때 true를 반환할 수 있다.
{: .prompt-general }

```cpp
class Solution {
public:
    bool isValidBST(TreeNode* root) {
        return dfs(root, LONG_MIN, LONG_MAX);
    }
    
    bool dfs(TreeNode* node, long small, long large) {
        if (node == nullptr) {
            return true;
        }
        
        if (small >= node->val || node->val >= large) {
            return false;
        }
        
        bool left = dfs(node->left, small, node->val);
        bool right = dfs(node->right, node->val, large);
        
        return left && right;
    }
};
```

함수의 반환 조건이 `AND` 조건이므로 모든 하위 트리도 BST여야 한다.

반복 버전:

```cpp
class Solution {
public:
    bool isValidBST(TreeNode* root) {
        stack<pair<TreeNode*, pair<long, long>>> stack;
        stack.push(pair(root, pair(LONG_MIN, LONG_MAX)));
        
        while (!stack.empty()) {
            auto[node, p] = stack.top();
            long small = p.first;
            long large = p.second;
            stack.pop();
            
            if (small >= node->val || node->val >= large) {
                return false;
            }
            
            if (node->left) {
                stack.push(pair(node->left, pair(small, node->val)));
            }
            
            if (node->right) {
                stack.push(pair(node->right, pair(node->val, large)));
            }
        }
        
        return true;
    }
};
```

시간 및 공간 복잡도는 다른 모든 예와 같은 이유로 $$O(n)$$이다.

---

출처: [Leetcode](https://leetcode.com/explore/interview/card/leetcodes-interview-crash-course-data-structures-and-algorithms/707/traversals-trees-graphs/4622/)

<!--

{: .prompt-general }

-->