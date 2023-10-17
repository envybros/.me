---
title: "이진 트리-DFS"
categories: [Algorithm 연구소]
tags: [Algorithm]
date: 2023-09-19 01:30
math: true
img_path: /assets/img/algorithm/
---

---

![Title](algorithm_title.png)

---

## **개요**

> 이 글을 시작하기 전에 이진 트리가 무엇인지, 코드에서 어떻게 표현되는지를 완전히 이해하는 것이 중요하다. 또한 재귀에 대한 확실한 이해도 필요하다.
{: .prompt-general }

이 글에서는 이진 트리를 **순회**하는 방법을 설명한다. 트리 순회는 트리의 요소에 접근하는 방법으로, 트리 문제를 해결하는 데 필수적이다.

이진 트리 순회의 기본 개념을 이해하기 위해, 링크드 리스트 순회 방식을 상기해보자. 링크드 리스트를 순회할 때는 다음과 같은 코드를 사용할 수 있었다:

```cpp
int getSum(ListNode* head) {
    int ans = 0;
    while (head != nullptr) {
        ans += head->val;
        head = head->next;
    }

    return ans;
}
```

이 코드는 리스트의 `head`부터 시작해서 각 노드를 방문하며 리스트에 있는 모든 값들의 합을 계산한다. 각 노드를 참조할 때는 `head` 변수를 사용하며, `.next` 속성을 통해 리스트를 순회한다.

이제 이진 트리를 순회하는 방법을 생각해보자. 이진 트리 순회는 링크드 리스트와 비슷한 원리로 동작하지만, `root`에서 시작하여 `.left`와 `.right` 포인터를 사용하여 트리의 각 노드를 방문한다. 링크드 리스트를 반복적 접근 방식을 통해 순회하는 것과 달리, 이진 트리 순회는 주로 재귀적으로 이루어진다.

트리를 탐색하는 방법에는 주로 두 가지가 있다. 하나는 깊이 우선 탐색(DFS)이며, 이 방식은 이진 트리에서 전위, 중위, 후위 순회와 같은 방법으로 수행될 수 있다. 또 다른 방법은 너비 우선 탐색(BFS)이다. 이 글에서는 먼저 DFS부터 살펴볼 것이다.

---

## **깊이 우선 탐색 (DFS)**

> 노드의 깊이는 루트 노드와의 거리를 의미한다.

DFS는 트리의 한 방향으로 가능한 한 깊숙이 들어가서(리프 노드에 도달할 때까지) 그 방향의 노드를 우선적으로 탐색한다. 예를 들어, 트리의 `node.left`를 우선으로 정하면, 왼쪽 자식 노드 방향으로 계속 탐색을 진행하고, 왼쪽 하위 트리를 모두 탐색한 후에야 오른쪽 하위 트리로 넘어간다.

트리(tree)라는 용어는 그 구조가 실제 나무를 닮았기 때문에 붙여진 이름이다. 이진 트리에서의 경로는 나무의 뿌리에서 **가지(branch)**로 퍼져나가는 형태를 가진다. DFS 방식은 한 방향의 가지를 따라 가능한 한 깊게 탐색하고, 해당 가지를 모두 탐색한 뒤에는 다른 가지로 되돌아가 탐색을 계속한다.

가지의 끝에 도달한 후, 탐색 지점을 되돌려야 하므로 DFS는 주로 재귀적 방법으로 구현하지만, 스택을 사용한 반복적 접근 방식을 통해 구현하기도 한다. 다음은 모든 노드를 방문하는 재귀적 DFS의 기본 예제이다.

> `dfs(node)` 함수를 호출할 때마다 해당 `node`를 방문하게 된다. 코드를 통해 확인할 수 있듯이, 오른쪽 자식 노드를 방문하기 전에 왼쪽 자식 노드를 먼저 방문한다.
{: .prompt-general }

```cpp
void dfs(TreeNode* node) {
    if (node == nullptr) {
        return;
    }
    
    dfs(node->left);
    dfs(node->right);
    return;
}
```

> 트리를 역방향으로 올라가는 방법에 대해 이해하기 어려워한다면, 재귀에 대한 설명이 있는 페이지를 다시 참고하는 것이 좋다. 또한 DFS 과정 중에는 여러 개의 재귀 호출이 스택에 쌓이게 되므로 이 점을 이해하고 있어야 한다.
{: .prompt-general }

DFS를 사용하는 알고리즘의 구조는 대부분 비슷하다는 것이 특징이다. 주로 다음의 단계를 따른다:

1. 종료 조건 처리: 이는 주로 빈 트리(즉, 노드가 `null`인 경우)에 대한 처리를 의미한다.
2. 현재 노드에 대한 처리를 수행한다.
3. 현재 노드의 자식 노드에 대해 재귀 호출을 수행한다.
4. 결과 반환

> 2단계와 3단계의 순서는 문제에 따라 바뀔 수 있으며, 이는 잠시 후에 자세히 설명하도록 한다.
{: .prompt-general }

이진 트리 문제를 해결할 때 중요한 점은, **각 함수 호출이 그 하위 트리에 대한 원래 문제의 '작은 버전'을 해결한다는 것**이다. 각 재귀 호출에서 수행하는 로직은 문제의 세부 사항에 따라 달라진다.

앞서 언급했듯이, DFS에는 세 가지 주요 유형이 있다. 이 유형들은 단계 2와 3의 실행 순서에 따라 구분된다.

다음으로, 특정 트리 구조를 기준으로, DFS의 유형을 자세히 살펴보자.

![Binary Tree3](binary_tree3.png)

### **전위 순회(Preorder traversal)**

전위 순회에서는 자식으로 이동하기 전에 현재 노드에서 로직이 수행된다. 트리의 각 노드의 값을 콘솔에 출력하고 싶다고 가정해 보자. 이 경우 특정 노드에서 현재 노드의 값을 출력한 다음 왼쪽 자식을 재귀적으로 호출하고, 그 후에 오른쪽 자식을 재귀적으로 호출한다.

```cpp
void preorderDfs(TreeNode* node) {
    if (node == nullptr) {
        return;
    }
    
    cout << node->val << endl; // 현재 노드 값 출력
    preorderDfs(node->left);   // 왼쪽 자식 호출
    preorderDfs(node->right);  // 오른쪽 자식 호출
    return;
}
```

예제 트리에서 이 코드를 실행하면 노드가 다음 순서로 출력된다: `0, 1, 3, 4, 6, 2, 5`.

로직(출력)은 각 함수 호출이 시작될 때 즉시 수행되므로 전위 순회는 함수 호출이 발생하는 순서대로 노드를 처리한다.

### **중위 순회 순회 (Inorder traversal)**

중위 순회는 먼저 왼쪽 자식을 재귀적으로 호출한 다음, 현재 노드에서 로직(이 경우, 출력)을 수행하고, 그 후에 오른쪽 자식을 재귀적으로 호출한다. 즉, 왼쪽 자식에 대한 호출이 로직 수행보다 우선하므로 왼쪽 자식이 없는 노드에 도달할 때까지 로직을 수행하지 않는다.

```cpp
void inorderDfs(TreeNode* node) {
    if (node == nullptr) {
        return;
    }

    inorderDfs(node->left);    // 왼쪽 자식 호출
    cout << node->val << endl; // 현재 노드 값 출력
    inorderDfs(node->right);   // 오른쪽 자식 호출
    return;
}
```

예제 트리에서 이 코드를 실행하면 노드가 다음 순서로 출력된다: `3, 1, 4, 6, 0, 2, 5`.

특정 노드의 경우, 왼쪽 하위 트리의 모든 값이 출력될 때까지 해당 노드의 값은 출력되지 않으며, 오른쪽 하위 트리의 값이 출력될 때까지 해당 노드의 값은 출력되지 않는다.

### **후위 순회 (Postorder traversal)**

후위 순회는 먼저 자식을 재귀적으로 호출한 다음 현재 노드에서 로직을 수행한다. 즉, 자식 노드들의 호출이 로직 수행보다 우선하므로 리프 노드에 도달할 때까지 로직이 수행되지 않는다. 후위 순회에서 루트는 로직이 수행되는 마지막 노드이다.

```cpp
void postorderDfs(TreeNode* node) {
    if (node == nullptr) {
        return;
    }

    postorderDfs(node->left);  // 왼쪽 자식 호출
    postorderDfs(node->right); // 오른쪽 자식 호출
    cout << node->val << endl; // 현재 노드 값 출력
    return;
}
```

예제 트리에서 이 코드를 실행하면 노드가 다음 순서로 출력된다: `3, 6, 4, 1, 5, 2, 0`.

특정 노드의 경우, 왼쪽 하위 트리의 모든 값이 출력될 때까지 오른쪽 하위 트리의 값은 출력되지 않으며, 그 후에 해당 노드의 값은 출력된다.

```text
각 순회의 이름은 현재 노드에서 로직이 수행되는 시점을 나타낸다:

전위(Pre) -> 자식들 이전

중위(In) -> 자식들 사이

후위(Post) -> 자식들 이후
```

---

## **DFS로 문제 풀기**

세 가지 유형의 DFS가 있지만, 이를 모두 알아야 한다는 걱정은 하지 않아도 된다. 재귀를 사용한다면, 이 세 가지 유형 사이를 전환하는 것은 매우 간단하다.

실제로 많은 문제에서 중요한 것은 DFS의 특정 유형을 사용하는 것이 아니라 모든 노드를 방문하는 것이다. 이 세 가지 유형의 차이점을 아는 것은 주로 퀴즈에서 유용할 수 있다.

이제 DFS를 사용하여 이진 트리 문제를 어떻게 해결할 수 있는지 몇 가지 예를 살펴보자.

---

> **예제 1**: [이진 트리의 최대 깊이](https://leetcode.com/problems/maximum-depth-of-binary-tree/)
>
> 이 문제에서는 이진 트리의 `root`가 주어지며, 루트에서 가장 먼 리프 노드까지의 최장 경로를 찾아야 한다.
{: .prompt-general }

먼저 재귀적 접근 방식을 사용해보자. 재귀 함수를 설계할 때는 종료 조건을 먼저 정의하는 것이 중요하다. 이 문제에서 종료 조건은 빈 트리가 주어졌을 때이며, 이 경우 깊이는 `0`이다.

이 문제를 해결하기 위해, 현재 노드에서의 최대 깊이를 찾아야 한다. 현재 노드의 왼쪽과 오른쪽 자식 노드 각각에 대해 함수를 재귀적으로 호출하고, 그 결과로 반환된 깊이 값 중 더 큰 값에 `1`을 더한다(현재 노드를 계산에 포함하기 위해). 이렇게 하면 현재 노드를 루트로 하는 하위 트리의 최대 깊이를 얻을 수 있다.

```cpp
/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode() : val(0), left(nullptr), right(nullptr) {}
 *     TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
 *     TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
 * };
 */
class Solution {
public:
    int maxDepth(TreeNode* root) {
        if (root == nullptr) {
            return 0;
        }
        
        int left = maxDepth(root->left);
        int right = maxDepth(root->right);
        return max(left, right) + 1;
    }
};
```

> 재귀와 관련된 중요한 점은 각 재귀 호출이 고유한 실행 컨텍스트를 가진다는 것이다. 각 재귀 호출은 각각의 `left`와 `right` 값을 가지며, 이 값들은 해당 노드의 하위 트리에 대한 정보를 포함한다.
{: .prompt-general }

재귀적으로 문제를 접근하는 것은 처음에는 다소 어려울 수 있지만, 이 방법을 익히면 이진 트리와 같은 재귀적 자료 구조를 다루는 데 매우 유용해질 수 있다.

### **반복적 접근 방식**

위의 솔루션에서는 현재 노드에 대한 로직이 호출 이후에 발생하기 때문에 후위 순회를 수행하고 있다. 세 가지 유형의 DFS는 모두 반복적 접근 방식으로 구현할 수 있지만, 후위 순회와 중위 순회는 전위 순회보다 구현하기 어려울 수 있다. 그러나 대부분의 문제에서 DFS의 특정 유형은 중요하지 않다. 여기서는 반복적 접근 방식을 사용한 전위 순회 DFS에 대해 설명할 것이다.

DFS를 반복적 접근 방식으로 구현하려면 스택을 사용해야 한다. 깊이 정보를 담는 반환 값이 없으므로 현재 깊이를 스택에 있는 각 노드 정보와 함께 저장해야 한다. 이 값을 연결하는 방법은 사용하는 프로그래밍 언어에 따라 다를 수 있다. Python과 같은 언어에서는 튜플을 사용하여 정보를 스택에 저장할 수 있다. 반면, Java와 같은 언어에서는 별도의 클래스나 두 개의 스택을 사용해야 할 수도 있다.

다음은 DFS를 반복문으로 구현하는 방법을 보여주는 예제 코드이다.

```cpp
/**
 * Definition for a binary tree node.
 * struct TreeNode {
 *     int val;
 *     TreeNode *left;
 *     TreeNode *right;
 *     TreeNode() : val(0), left(nullptr), right(nullptr) {}
 *     TreeNode(int x) : val(x), left(nullptr), right(nullptr) {}
 *     TreeNode(int x, TreeNode *left, TreeNode *right) : val(x), left(left), right(right) {}
 * };
 */
class Solution {
public:
    int maxDepth(TreeNode* root) {
        if (root == nullptr) {
            return 0;
        }
        
        stack<pair<TreeNode*, int>> stack;
        stack.push(pair(root, 1));
        int ans = 0;
        
        while (!stack.empty()) {
            auto [node, depth] = stack.top();
            stack.pop();
            ans = max(ans, depth);
            
            if (node->left != nullptr) {
                stack.push(pair(node->left, depth + 1));
            }
            
            if (node -> right != nullptr) {
                stack.push(pair(node->right, depth + 1));
            }
        }
        
        return ans;
    }
};
```

> 재귀 방식에 익숙하지 않은 사람들에게는 이 반복적 접근 방식이 더 직관적으로 느껴질 수 있다.
>
> 각 노드와 그 노드의 깊이 정보를 연결하여 사용한다. 주어진 노드에 대한 깊이가 depth일 경우, 그 자식 노드의 깊이는 depth + 1이 된다.
>
> 이 반복적 접근 방식은 다양한 문제에 적용할 수 있어 유용하다. 이 방법은 스택을 사용하며, 스택이 비어 있지 않은 동안 while 루프를 계속 실행한다. 이 루프는 각 반복에서 한 노드씩 처리하며, 이 과정은 재귀 호출의 동작과 유사하다.
{: .prompt-general }

트리 관련 문제의 시간 및 공간 복잡도는 일반적으로 상대적으로 간단한 편이다. 각 노드를 한 번씩 방문하며, 각 노드에서 상수 시간에 해당하는 작업을 수행하기 때문에 시간 복잡도는 대체로 $$O(n)$$이다. 여기서 $$n$$은 트리의 노드 수이다. 만약 각 노드에서 여러 작업을 수행해야 하는 경우, 예를 들어 $$O(k)$$의 작업을 수행한다면, 전체 시간 복잡도는 $$O(n \cdot k)$$가 될 것이다.

공간 복잡도 측면에서도, 재귀를 사용하는 경우에도 호출 스택이 메모리를 차지하므로 이 부분도 고려해야 한다. 스택의 최대 크기는 트리의 구조에 따라 달라진다. 트리가 한 줄기로 이루어진 경우, 즉 한 노드가 하나의 자식만을 가진 경우, 스택의 크기는 $$O(n)$$이 될 수 있다. 반면, 트리가 완전 이진 트리인 경우, 스택의 크기는 $$O(\log{}n)$$이 될 수 있다.

> 반복적 접근 방식에서 주의해야 할 점으로, 코드에서 왼쪽 자식 노드(`node.left`)를 오른쪽 자식 노드(`node.right`)보다 먼저 스택에 추가한다는 것이 있다. 이는 스택이 후입선출(LIFO) 구조이기 때문에, 오른쪽 자식 노드를 먼저 추가하면 왼쪽 자식 노드를 먼저 처리하게 되어 전위 순회 순서를 유지할 수 있다.
{: .prompt-general }

---

> **예제 2**: [경로 합](https://leetcode.com/problems/path-sum/)
>
> 이진 트리의 `root`와 정수 `targetSum`이 주어졌을 때, 루트에서 리프까지의 경로 상에 있는 노드의 합이 targetSum과 동일한 경로가 존재하면 `true`를 반환하고, 그렇지 않으면 `false`를 반환한다.
{: .prompt-general }

먼저, 각 함수 호출에 필요한 정보는 무엇일까? 당연히 현재 노드 정보는 필요하지만, 다른 정보도 추가로 필요할 수 있다. 루트에서 현재 노드까지의 경로 상에 있는 노드의 합계를 나타내는 정수 `curr`이 필요하다. 리프 노드에 도달했을 때, 이 값과 `targetSum`을 비교하여 경로의 합계가 요구 조건을 만족하는지 확인할 수 있다. 따라서, 현재 노드에서 시작하여 특정 노드에서 끝나는 경로의 합계가 `targetSum`과 같고, 이미 그 합계에 기여한 `curr` 값이 있는 경우 `true`를 반환하는 보조 함수 `dfs(node, curr)`가 필요하다.

종료 조건은 무엇일까? 트리가 비어 있으면 `false`를 반환해야 한다. 왜냐하면 노드가 없으므로 유효한 경로도 존재하지 않기 때문이다. 리프 노드에 도달한 경우(즉, 노드의 양쪽 자식이 모두 `null`인 경우), `curr`과 `node`의 값의 합이 `targetSum`과 같은지 확인한 후 결과를 반환한다.

리프 노드가 아닌 경우, 왼쪽 또는 오른쪽 자식 노드로 경로를 계속 탐색할 수 있다. `targetSum`과 동일한 합계를 가진 경로를 찾기 위해서는 한 쪽 경로의 탐색 결과가 `true`이면 충분하다. 당연히, `curr` 값에 현재 노드의 값을 더하는 것을 잊지 않아야 한다.

```cpp
class Solution {
public:
    int target;
    
    bool hasPathSum(TreeNode* root, int targetSum) {
        target = targetSum;
        return dfs(root, 0);
    }
    
    bool dfs(TreeNode* node, int curr) {
        if (node == nullptr) {
            return false;
        }
        
        if (node->left == nullptr && node->right == nullptr) {
            return (curr + node->val) == target;
        }
        
        curr += node->val;
        bool left = dfs(node->left, curr);
        bool right = dfs(node->right, curr);
        return left || right;
    }
};
```

반복적 접근 방식은 다음과 같다. 반복적 접근은 재귀적 접근보다 덜 직관적일 수 있으므로, 면접 상황에서 요청하지 않는 한 사용을 자제하는 것이 좋다.

```cpp
class Solution {
public: 
    bool hasPathSum(TreeNode* root, int targetSum) {
        if (root == nullptr) {
            return false;
        }
        
        stack<pair<TreeNode*, int>> stack;
        stack.push(pair(root, 0));
        
        while (!stack.empty()) {
            auto [node, curr] = stack.top();
            stack.pop();

            if (node->left == nullptr && node->right == nullptr) {
                if (curr + node->val == targetSum) {
                    return true;
                }
            }
            
            curr += node->val;
            if (node->left != nullptr) {
                stack.push(pair(node->left, curr));
            }
            
            if (node->right != nullptr) {
                stack.push(pair(node->right, curr));
            }
        }
        
        return false;
    }
};
```

모든 노드는 최대 한 번씩 방문되며, 각 방문은 일정한 처리 시간을 필요로 하기 때문에, 시간 복잡도는 $$O(n)$$이다. 여기서 n은 트리의 노드 수를 나타낸다. 최악의 경우 공간 복잡도도 $$O(n)$$이 될 수 있으며, 이는 재귀 호출의 깊이가 노드 수에 비례할 수 있기 때문이다.

---

> **예제 3**: [이진 트리에서 좋은 노드 개수 구하기](https://leetcode.com/problems/count-good-nodes-in-binary-tree/)
>
> 이진 트리의 `root`가 주어졌을 때, **좋은** 노드의 개수를 세는 문제다. 노드가 좋은 노드로 분류되기 위해서는 루트부터 해당 노드에 이르는 경로 상의 다른 노드들의 값보다 그 값이 크거나 같아야 한다.
{: .prompt-general }

첫 단계로, 각 함수 호출에 필요한 정보를 파악하자. 노드가 **좋은 노드**인지 판단하려면 현재 노드의 값과 지금까지의 경로 상에서 만난 노드들의 최댓값을 알아야 한다. 이 정보를 담기 위해, 현재까지 본 노드들 중 가장 큰 값을 저장하는 변수를 `maxSoFar`라고 부르겠다.

다음으로, 현재 노드를 **`root`로 하는 하위 트리**에서 좋은 노드의 개수를 반환하는 함수를 작성할 것이다. 이 함수는 `dfs(node, maxSoFar)` 형태를 가지며, `maxSoFar`는 지금까지 본 노드의 값 중 최댓값을 저장한다.

종료 조건는 무엇일까? 트리가 비어있다면, 즉 노드가 없다면 좋은 노드도 없으므로 답은 `0`이 된다.

하나의 하위 트리에서 **좋은 노드**의 총 개수는 현재 노드가 **좋은 노드**라면 1을 더하고, 왼쪽 하위 트리와 오른쪽 하위 트리에서 **좋은 노드**의 개수를 각각 더한 값이다. 현재 노드가 **좋은 노드**인지는 `node->val`이 `maxSoFar` **이상**인지로 판단한다. 이후 왼쪽과 오른쪽 하위 트리에서 **좋은 노드**가 몇 개 있는지 찾아야 하며, 이 과정에서 `maxSoFar`을 업데이트해야 한다.

```cpp
class Solution {
public:
    int goodNodes(TreeNode* root) {
        return dfs(root, INT_MIN);
    }
    
    int dfs(TreeNode* node, int maxSoFar) {
        if (node == nullptr) {
            return 0;
        }
        
        int left = dfs(node->left, max(maxSoFar, node->val));
        int right = dfs(node->right, max(maxSoFar, node->val));
        int ans = left + right;
        if (node->val >= maxSoFar) {
            ans++;
        }
        
        return ans;
    }
};
```

반복적 접근 방식도 가능하다.

```cpp
class Solution {
public:
    int goodNodes(TreeNode* root) {
        if (root == nullptr) {
            return 0;
        }
        
        stack<pair<TreeNode*, int>> stack;
        stack.push(pair(root, INT_MIN));
        int ans = 0;
        
        while (!stack.empty()) {
            auto [node, maxSoFar] = stack.top();
            stack.pop();
            
            if (node->val >= maxSoFar) {
                ans++;
            }
            
            if (node->left) {
                stack.push(pair(node->left, max(maxSoFar, node->val)));
            }
            
            if (node->right) {
                stack.push(pair(node->right, max(maxSoFar, node->val)));
            }
        }
        
        return ans;
    }
};
```

시간 및 공간 복잡도는 트리의 노드 수에 비례하여 $$O(n)$$이다. 이전 예제와 마찬가지로 모든 노드를 한 번씩 방문해야 하기 때문이다.

---

> **예제 4**: [같은 트리](https://leetcode.com/problems/same-tree/)
>
> 두 이진 트리 `p`와 `q`의 루트가 주어졌을 때, 두 트리가 같은 트리인지 확인한다. 두 이진 트리가 구조적으로 동일하고 노드의 값이 같으면 동일한 트리이다.
{: .prompt-general }

이 문제는 이진 트리의 재귀적 특성을 잘 보여준다.

`p`와 `q`가 같은 트리이려면 다음이 참이어야 한다:

1. `p`와 `q`의 값은 같다 (`p.val == q.val`).
2. `p.left`와 `q.right`는 같은 트리여야 한다.
3. `p.right`와 `q.right`는 같은 트리여야 한다.

주요 아이디어는 두 트리가 같다면 그 하위 트리도 같아야 한다는 것이다. 이것은 문제에 대한 재귀적 정의를 제공한다. 우리가 구현하려는 함수는 두 트리가 동일한지 여부를 알려주는 것이므로, 함수 자체를 사용하여 조건 2와 3을 검사할 수 있다.

다음 조건을 사용하여 p와 q가 같은 트리인지 확인할 수 있다:

`p.val == q.val && isSameTree(p.left, q.left) && isSameTree(p.right, q.right)`

이제 재귀가 결국 종료되도록 종료 조건(Base case)이 필요하다. `p`와 `q`가 모두 `null`인 경우, 기술적으로 둘 다 동일한(빈) 트리이므로 `true`를 반환할 수 있다. `p`나 `q` 중 하나만 `null`이고 다른 하나는 `null`이 아닌 경우, 둘은 분명히 같은 트리가 아니므로 `false`를 반환해야 한다.

종료 조건을 생각하는 좋은 방법은 노드가 하나뿐인 트리를 생각해 보는 것이다. `p`와 `q`가 모두 같은 값을 가진 하나의 노드 트리라고 가정해 보자. 첫 번째 부울 검사 `p.val == q.val`이 통과되었으므로 이제 하위 트리를 검사한다. 노드에 자식이 없으므로 왼쪽 및 오른쪽 하위 트리에 대한 호출은 모두 종료 조건을 트리거하고 `true`를 반환한다.

> 재귀의 장점은 루트에 있을 때 왼쪽과 오른쪽 하위 트리에 수천 개의 노드가 있을 수 있음에도 불구하고 실제로 트리를 통과하는 과정에는 많은 계단식 호출이 있지만, 호출만 하면 필요한 결과를 얻을 수 있으므로 걱정할 필요가 없다는 것이다.
{: .prompt-general }

```cpp
class Solution {
public:
    bool isSameTree(TreeNode* p, TreeNode* q) {
        if (p == nullptr && q == nullptr) {
            return true;
        }
        
        if (p == nullptr || q == nullptr) {
            return false;
        }
        
        if (p->val != q->val) {
            return false;
        }
        
        bool left = isSameTree(p->left, q->left);
        bool right = isSameTree(p->right, q->right);
        return left && right;
    }
};
```

다음은 반복적 접근 방식을 사용한 접근 방식이다. 케이스를 확인하기 위해 같은 코드를 사용하고 있음을 주목해보자. 순회하는 동안 true를 반환하는 대신 조건이 깨지면 false를 반환하고, false를 반환하지 않고 트리를 통과할 수 있으면 마지막에 true를 반환한다.

```cpp
class Solution {
public:
    bool isSameTree(TreeNode* p, TreeNode* q) {
        stack<pair<TreeNode*, TreeNode*>> stack;
        stack.push(pair(p, q));
        
        while (!stack.empty()) {
            auto [p, q] = stack.top();
            stack.pop();
            
            if (p == nullptr && q == nullptr) {
                continue;
            }

            if (p == nullptr || q == nullptr) {
                return false;
            }

            if (p->val != q->val) {
                return false;
            }
            
            stack.push(pair(p->left, q->left));
            stack.push(pair(p->right, q->right));
        }

        return true;
    }
};
```

시간 및 공간 복잡도는 다른 예와 같은 이유로 모두 $$O(n)$$이다.

---

> **참고**: 다음 문제와 해결 방법을 이해하는 데 어려움이 있더라도 걱정하지 않아도 된다. 이 문제는 이전에 다룬 다른 예제들보다 난이도가 높아서 곧 제외될 예정이다.
>
> [이 문제](https://en.wikipedia.org/wiki/Lowest_common_ancestor)는 자주 출제되기 때문에 코스에서 '보너스'로 남겨두었다.
>
> 보너스 예: [236. 이진 트리의 최하위 공통 조상](https://leetcode.com/problems/lowest-common-ancestor-of-a-binary-tree/)
>
> 이진 트리의 `root`와 트리 내의 두 노드 `p`와 `q`가 주어지면, 두 노드의 **최소 공통 조상(LCA)**을 찾아 반환하라. LCA란 트리에서 `p`와 `q`를 모두 자손으로 갖는 가장 낮은(가장 가까운) 노드를 의미한다(참고: 노드는 자신의 자손으로 간주한다).
{: .prompt-general }

이 문제는 고전적이며, 이전에 본 문제들보다 조금 더 까다롭다. 다시 한 번, 재귀 함수를 사용하여 이 질문에 대한 답을 찾길 원한다. 그렇다면 종료 조건은 무엇일까? 만약 트리가 비어있다면 LCA가 존재하지 않으므로 `null`을 반환한다.

그렇지 않다면 어떤 노드가 LCA인지 어떻게 알 수 있을까? 우리가 루트에 있다고 가정하면 세 가지 가능성이 있다.

1. 루트 노드 자체가 `p` 또는 `q`인 경우. 이 경우, 답은 루트 노드 아래에 있을 수 **없다**. 그 이유는 그러면 루트의 자식 노드로서 `p` 또는 `q` 중 하나가 누락되기 때문이다.
2. `p`와 `q` 중 하나는 왼쪽 하위 트리에 있고, 다른 하나는 오른쪽 하위 트리에 있다. 이 경우 루트는 두 하위 트리 사이의 연결점이므로, `p`와 `q`를 모두 자손으로 갖는 가장 낮은 노드가 되어야 한다.
3. `p`와 `q` 모두 하나의 하위 트리에 위치하고 있다. 이 경우, 그 하위 트리 안을 조사하여 "더 낮은" 노드를 찾을 수 있으므로, 루트는 답이 아니다.

중요한 점은, 트리의 재귀적 성질 때문에 이러한 경우를 알고리즘으로 변환할 수 있다는 것이다. 첫 번째 또는 세 번째 경우에 대한 답을 찾는 방법만 알아내면 된다.

첫 번째 경우에서, 현재 노드가 `p` 또는 `q`인 경우, 하위 트리에 답이 있을 수 없다는 것을 알고 있으므로, 하위 트리에 대해 걱정할 필요가 없다. 따라서 무언가를 바로 반환할 것이다. 종료 조건에서는 `null`을 반환한다. 따라서 하위 트리를 호출할 때 `p` 또는 `q` 중 하나가 그 하위 트리에 있다면 null이 아닌 것을 반환한다.

그런 다음, 왼쪽과 오른쪽 하위 트리에 대한 호출 모두에서 무언가를 반환하면, 두 번째 경우가 되고, 호출 중 하나만 무언가를 반환하면 세 번째 경우를 의미한다.

```cpp
class Solution {
public:
    TreeNode* lowestCommonAncestor(TreeNode* root, TreeNode* p, TreeNode* q) {
        if (root == nullptr) {
            return nullptr;
        }
        
        if (root == p || root == q) {
            return root;
        }
        
        TreeNode* left = lowestCommonAncestor(root->left, p, q);
        TreeNode* right = lowestCommonAncestor(root->right, p, q);
        
        if (left != nullptr && right != nullptr) {
            return root;
        }
        
        if (left != nullptr) {
            return left;
        }
        
        return right;
    }
};
```

이 알고리즘은 각 노드를 최대 한 번씩 방문하므로, 일반적인 시간 복잡도는 $$O(n)$$이다. 재귀 호출 스택은 최대 $$O(n)$$의 공간을 사용할 수 있다.

이 문제를 반복적 접근방식으로 해결하는 것도 가능하나, 그 방법은 훨씬 더 어렵고 복잡하다.

여기까지가 이번 섹션에서 다룰 모든 내용이다. 다음 주제로 넘어가기 전에, 다음 몇 가지 연습 문제를 시도해 보는 것이 좋다.

---

출처: [Leetcode](https://leetcode.com/explore/interview/card/leetcodes-interview-crash-course-data-structures-and-algorithms/707/traversals-trees-graphs/4686/)

<!--

{: .prompt-general }

-->