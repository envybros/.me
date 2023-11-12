---
title: "빠른 포인터와 느린 포인터"
categories: [Algorithm 연구소]
tags: [Algorithm]
date: 2023-09-14 01:30
math: true
img_path: /assets/img/algorithm/
---

---

![Title](algorithm_title.png)

---

## **개요**

빠른 포인터와 느린 포인터는 주로 배열이나 문자열에서 활용되는 두 포인터 기법의 한 예이다. 이 기법의 핵심은 두 포인터가 동일한 속도로 움직이지 않는다는 점이다. 즉, 반복문을 진행하면서 포인터들이 서로 다른 "속도"로 이동하거나, 서로 다른 위치에서 시작하거나, 기타 여러 방식으로 조작될 수 있다.

일반적으로 "빠른" 포인터는 한 번 반복할 때마다 두 개의 요소를 넘어간다. 반면 "느린" 포인터는 한 번 반복할 때마다 한 개의 요소만 넘어간다. 이는 항상 그렇다는 건 아니지만, 대부분의 경우에 그렇다. 아래에 이를 나타내는 의사 코드이다:

```cpp
// head는 링크드 리스트의 첫 번째 노드이다
function fn(head):
    slow = head
    fast = head

    while fast and fast.next:
        여기서 어떤 처리를 진행
        slow = slow.next
        fast = fast.next.next
```

> while 반복문에서 `fast.next`를 확인하는 이유는, `fast` 포인터가 리스트의 마지막 노드에 도달하면 `fast.next`는 `null`이 되어, `fast.next`에 접근하려고 하면 오류가 발생하기 때문이다. (즉, `null.next`는 오류를 일으킨다.)
{: .prompt-general }

빠른 포인터와 느린 포인터 기법이 어떻게 유용한지 몇 가지 예를 통해 알아보자.

---

> **예제 1**: 링크드 리스트의 `head` 노드가 주어지고, 이 리스트의 노드 수가 **홀수**일 때 중간에 있는 노드의 값을 반환하는 경우.
>
> 예를 들어, `1 -> 2 -> 3 -> 4 -> 5` 구조를 가진 링크드 리스트가 있을 때, 이 방법을 사용하면 `3`을 반환한다.
{: .prompt-general }

이 문제를 해결하는 한 가지 간단한 방법은 링크드 리스트를 순회하면서 모든 값을 배열에 저장한 다음, 배열의 중간 값을 반환하는 것이다.

```cpp
function fn(head):
    array = int[]
    while head:
        array.push(head.val)
        head = head.next

    return array[array.length]  // array.length: 2
```

그러나 이 방법은 기본적으로 "트릭"에 불과하며, 실제 면접 상황에서는 바람직한 해결책으로 받아들여지지 않을 것이다. 이 문제의 복잡성은 링크드 리스트의 전체 길이를 사전에 알 수 없기 때문에 발생한다. 따라서 전체 길이를 알아내기 위해 리스트를 한 번 순회한 다음, 그 길이를 바탕으로 다시 한 번 순회하여 중간 지점을 찾아야 한다.

```cpp
int getMiddle(ListNode* head) {
    int length = 0;
    ListNode* dummy = head;
    while (dummy != nullptr) {
        length++;
        dummy = dummy->next;
    }

    for (int i = 0; i < length / 2; i++) {
        head = head->next;
    }

    return head->val;
}
```

하지만 더 우아한 해결책은 빠른 포인터와 느린 포인터를 사용하는 것이다. 한 포인터가 다른 포인터보다 두 배 빠르게 이동하면, 느린 포인터는 리스트의 중간 지점 근처에 도달할 때 빠른 포인터는 리스트의 끝에 도달한다.

```cpp
int getMiddle(ListNode* head) {
    ListNode* slow = head;
    ListNode* fast = head;
    while (fast != nullptr && fast->next != nullptr) {
        slow = slow->next;
        fast = fast->next->next;
    }

    return slow->val;
}
```

이 기법은 $$O(1)$$의 추가 공간만을 사용하며, 링크드 리스트에 노드가 있을 경우 $$O(n)$$의 시간 복잡도로 리스트를 순회한다.

---

> **예제 2**: [링크드 리스트 사이클](https://leetcode.com/problems/linked-list-cycle/)
>
> 링크드 리스트의 `head`가 주어졌을 때, 링크드 리스트에 사이클이 있는지 확인한다.
>
> 리스트에서 `next` 포인터를 계속 따라가면 동일한 노드로 다시 돌아올 수 있는 경우, 그 링크드 리스트에는 사이클이 존재한다고 말할 수 있다.
{: .prompt-general }

링크드 리스트에 사이클이 있다면, 노드들이 원형을 이루며 끝없이 반복한다고 생각할 수 있다. 이러한 문제를 해결하기 위한 한 가지 방법은 링크드 리스트를 여러 번 반복 실행하는 것이다. 사이클이 없다면, 결국 리스트의 끝에 도달할 것이다. 그러나 사이클이 있다면, 무한히 반복할 것이고, 결국 사이클이 존재한다고 결론지을 수 있다.

그러나 이 방법에는 문제점이 있다. 이는 실제로는 논리적인 해결책이 아니다. 만약 리스트에 사이클이 없고, 그 길이가 설정한 반복 횟수보다 길다면 이 방법은 실패할 것이다. 반복 횟수를 늘리면 더 긴 링크드 리스트에서도 작동할 것이라 주장할 수 있지만, 이는 실제로는 실행 가능한 해결책이 아니다. 특정 수치를 크게 설정하면 실용적이지 않을 뿐더러, 고정된 마법의 숫자에 의존하는 것은 좋지 않은 관행이다.

더 합리적인 방법은 빠른 포인터와 느린 포인터를 사용하는 것이다. 직선 트랙에서 두 명의 달리기 선수가 경주한다고 상상해보자. 한 명은 현저히 빠르고 다른 한 명은 느리다. 느린 선수는 결코 빠른 선수를 따라잡을 수 없다. 빠른 선수가 결승선을 통과하는 것은 링크드 리스트의 끝에 도달한 것과 같다.

하지만 선수들이 단순히 직선 경로가 아니라 원형 트랙에서 여러 바퀴를 뛴다면 어떨까? 이 경우, 빠른 선수는 결국 느린 선수를 한 바퀴 이상 앞질러 나가게 될 것이다.

이 원리를 이용하면, 빠른 포인터를 느린 포인터보다 두 배 빠른 속도로 이동시킬 수 있다. 두 포인터가 만나면 사이클이 존재한다고 판단할 수 있다. 만약 빠른 포인터가 리스트의 끝에 도달한다면, 그 리스트는 사이클이 없다고 볼 수 있다.

> 하지만 왜 포인터들이 항상 만나는 걸까? 빠른 포인터가 사이클을 돌 때 왜 느린 포인터를 그냥 건너뛰지 않을까? 이는 사이클을 한 번 돌고 나면, 빠른 포인터가 느린 포인터보다 한 칸 뒤에 오게 되고, 그 다음 사이클에서는 두 포인터가 만나게 되기 때문이다. 빠른 포인터가 두 칸 뒤에 있으면, 다음 사이클에서 한 칸 뒤에 오게 된다. 이런 패턴이 계속되어, 빠른 포인터는 각 사이클마다 느린 포인터에 한 단계씩 더 가까이 다가서므로, "건너뛰는" 일은 발생하지 않는다.
{: .prompt-general }

![LinkedList2](LinkedList2.png)

```cpp
class Solution {
public:
    bool hasCycle(ListNode *head) {
        ListNode* slow = head;
        ListNode* fast = head;
        while (fast != nullptr && fast->next != nullptr) {
            slow = slow->next;
            fast = fast->next->next;
            if (slow == fast) {
                return true;
            }
        }
        
        return false;
    }
};
```

이 방법의 시간 복잡도는 $$O(n)$$이고, 공간 복잡도는 $$O(1)$$이며, 여기서 $$n$$은 링크드 리스트의 노드 수를 나타낸다. 해싱을 사용하여 이 문제를 해결할 수도 있지만, 그러면 $$O(n)$$의 추가 공간이 필요하다.

해싱 방법을 사용한 해결책: `next` 포인터를 계속 따라가며 반복할 때, 두 가지 상황이 발생할 수 있다:

1. 링크드 리스트에 사이클이 없다면, 결국 `null`에 도달하게 되어 순회가 종료된다.
2. 링크드 리스트에 사이클이 있다면, 어떤 노드를 반드시 두 번 방문하게 될 것이다. 이를 감지하기 위해 우리는 집합을 사용할 수 있다.

```cpp
class Solution {
public:
    bool hasCycle(ListNode *head) {
        unordered_set<ListNode*> seen;
        while (head != nullptr) {
            if (seen.find(head) != seen.end()) {
                return true;
            }
            seen.insert(head);
            head = head->next;
        }
        
        return false;
    }
};
```

이 방법은 간결하나, 첫 번째 방법보다 공간 효율성이 떨어지므로, 첫 번째 방법이 더 선호된다.

---

> **예제 3**: 링크드 리스트의 헤드와 정수 `k`가 주어지면, 끝에서 $$k^th$$번째 노드를 반환한다.
>
> 예를 들어, `1 -> 2 -> 3 -> 4 -> 5`를 나타내는 링크드 리스트가 있고, `k = 2`라고 가정하면, 끝에서 두 번째 노드이므로 값이 `4`인 노드를 반환한다.
{: .prompt-general }

이 문제도 첫 번째 예제와 매우 유사하다. 다시 말해, 리스트를 배열로 변환하거나 한 번 반복하여 길이를 찾은 다음, 길이를 알면 다시 반복할 수 있다. 그러나 더 우아한 해결책이 존재한다.

두 포인터를 `k` 간격으로 분리한 다음, 같은 속도로 이동하면 두 포인터는 항상 `k` 간격으로 떨어져 있다. 빠른 포인터(더 앞쪽에 있는 포인터)가 끝에 도달하면, 느린 포인터는 끝에서 `k`번째 노드에 위치하게 된다.

```cpp
ListNode* findNode(ListNode* head, int k) {
    ListNode* slow = head;
    ListNode* fast = head;
    for (int i = 0; i < k; i++) {
        fast = fast->next;
    }

    while (fast != nullptr) {
        slow = slow->next;
        fast = fast->next;
    }
    
    return slow;
}
```

이 알고리즘의 시간 복잡도는 $$O(n)$$, 공간 복잡도는 $$O(1)$$이며, 여기서 n은 링크드 리스트의 노드 수이다.

빠른 포인터와 느린 포인터 기법을 사용하여 문제를 해결해보자. 이 기법은 링크드 리스트를 처리할 때 유용한 패턴 중 하나이다.

---

출처: [Leetcode](https://leetcode.com/explore/interview/card/leetcodes-interview-crash-course-data-structures-and-algorithms/704/linked-lists/4507/)
