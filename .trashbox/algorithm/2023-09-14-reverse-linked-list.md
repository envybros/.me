---
title: "링크드 리스트 뒤집기"
categories: [Algorithm 연구소]
tags: [Algorithm]
date: 2023-09-14 02:30
math: true
img_path: /assets/img/algorithm/
---

---

![Title](algorithm_title.png)

---

## **개요**

링크드 리스트를 뒤집는 것은 면접 문제로 자주 나오는 것이며, 다른 문제를 해결하는 데에도 사용될 수 있는 기법이다. 이를 우아하게 수행하기 위해서는 포인터를 이동하는 방법을 잘 알아야 하며, 이 글에서는 그 방법을 설명한다.

예를 들어, 링크드 리스트 `1 -> 2 -> 3 -> 4`를 뒤집어 `4 -> 3 -> 2 -> 1`로 만들고 싶다고 가정해보자. 현재 노드를 나타내는 포인터인 `curr`가 `1`을 가리키고 있다고 하자. 이제 `2`가 `curr`를 가리킬 수 있도록 해야 한다. 하지만 `curr`를 다음으로 이동(`curr = curr.next`)시키면, 단일 링크드 리스트이기 때문에 `1`에 대한 참조를 잃어버린다. 이 문제를 해결하기 위해서는 이전 노드를 가리키는 추가 포인터인 `prev`가 필요하다.

`curr` 위치에서 `curr.next = prev`를 수행하여 화살표의 방향을 바꿀 수 있다. 이후 `prev`를 `curr`로 업데이트하여 다음 노드의 준비를 할 수 있다. 그러나 `curr.next`를 바꾸면, 원래의 다음 노드에 대한 참조를 잃어버리게 된다. 이 문제를 해결하기 위해, 다음 노드를 가리키는 임시 변수 `nextNode`를 사용하여 다른 포인터를 업데이트하기 전에 그 참조를 저장할 수 있다.

```cpp
ListNode* reverseList(ListNode* head) {
    ListNode* prev = nullptr;
    ListNode* curr = head;
    while (curr != nullptr) {
        ListNode* nextNode = curr->next; // 먼저, 다음 노드를 잃어버리지 않도록 한다
        curr->next = prev;               // 포인터의 방향을 바꾼다
        prev = curr;                     // 다음 노드의 준비를 위해 현재 노드를 prev로 설정한다
        curr = nextNode;                 // 계속 진행한다
    }

    return prev;
}

```

이 알고리즘의 시간 복잡도는 $$O(n)$$이며, 여기서 n은 링크드 리스트의 노드 수이다. while 루프는 $$n$$번 실행되고, 각 반복은 상수 시간 $$O(1)$$이 걸린다. 추가적인 저장 공간을 사용하지 않기 때문에 공간 복잡도는 $$O(1)$$이다.

링크드 리스트 문제는 종종 직관적이고 우아한 해결책을 가지고 있다. 문제를 한 단계씩 해결하면서 필요한 것이 무엇인지를 생각하는 것이 중요하다. 위의 예에서 다음과 같은 과정을 거쳤다:

1. `curr`이 있는 위치에서 다음 포인터를 이전 노드로 설정해야 한다.
    - `prev` 포인터를 사용하여 이전 노드를 추적한다.
2. 반복할 때마다 `prev` 포인터를 업데이트해야 한다.
    - `curr.next`를 업데이트한 후에 `prev = curr`로 설정하여 다음 노드를 위해 준비한다.
3. `curr.next = prev`로 설정하면 원래 `curr.next`를 잃게 된다.
    - `nextNode`를 사용하여 원래 `curr.next`의 참조를 유지한다.

이러한 지침을 코드로 쉽게 전환할 수 있고, 이를 통해 효율적인 알고리즘을 작성할 수 있다.

다른 예를 살펴보자.

---

> 예제: [쌍으로 노드 스왑](https://leetcode.com/problems/swap-nodes-in-pairs/)
>
> 링크드 리스트의 `head`가 주어지면 노드 쌍을 바꿔보자. 예를 들어, `1 -> 2 -> 3 -> 4 -> 5 -> 6`의 링크드 리스트가 주어지면, `2 -> 1 -> 4 -> 3 -> 6 -> 5`의 링크드 리스트를 반환한다.
{: .prompt-general }

이 문제도 단계별로 분해하여 생각해보자. 첫 번째 노드 쌍을 `A -> B`라고 가정한다.

1. A 노드에서 시작하여 B가 A를 가리킬 수 있도록 해야 한다.
    - `head.next.next = head`를 사용하면 된다.
2. 그러나 `B.next`를 변경하면 나머지 리스트에 액세스할 수 없게 된다.
    - 1단계의 변경 사항을 적용하기 전에 `nextNode = head.next.next` 포인터를 저장한다.

> head.next.next는 1단계와 2단계에서 다르게 사용된다. 할당 연산자(=) 앞에 있을 때는 head.next의 다음 노드를 **변경**한다. **할당** 연산자(=) 뒤에 있으면 head.next의 다음 노드를 참조한다.
{: .prompt-general }

1. 이제 `B`가 `A`를 가리키고 있다. 다음 쌍인 `C, D`로 넘어가야 하지만, `A`는 여전히 `B`를 가리키고 있어 원하는 방향이 아니다. 바로 다음 쌍으로 넘어가면 `A`에 대한 참조를 잃게 되고 `A.next`를 변경할 수 없게 된다.
    - `prev = head`로 다른 포인터에 `A`를 저장한다(아직 `head`를 변경하지 않았으므로 여전히 `A`를 가리키고 있다).
    - 다음 쌍으로 이동하려면 `head = nextNode`를 수행한다.
2. 다음 쌍인 `C -> D`로 넘어가려면 `A`가 `D`를 가리켜야 한다.
    - 이제 `head`가 `C`에 있고 `prev`가 `A`에 있으므로 `prev.next = head.next`를 사용하면 된다.
3. 첫 번째 쌍 `A, B`가 완전히 완성되었다. `B`는 `A`를 가리키고 `A`는 `D`를 가리킨다. 처음 시작할 때는 `head`가 `A`를 가리키고 있었지만, 1~4 단계를 거쳐 `A, B`를 완성했다. 이제 `head`가 `C`를 가리키고 있다. 단계를 다시 진행하면 `C, D`를 완성하고 다음 쌍을 준비할 수 있다. 모든 쌍이 바뀔 때까지 여러 단계를 반복하면 된다. 하지만 마지막에 돌아오는 것은 무엇일까?
    - 모든 쌍이 완료되면 `B`를 반환해야 하는데, 그전에 `B`에 대한 참조를 잃어버렸다.
    - 알고리즘을 시작하기 전에 `dummy` 노드에 `B`를 저장하면 이 문제를 해결할 수 있다.
4. 노드 수가 홀수이면 어떻게 할까? 4단계에서 `A.next`를 `C.next`로 설정했다. 만약 노드가 3개만 있어서 `C.next`가 null이면 어떨까?
    - 다음 쌍으로 넘어가기 전에 `head.next = nextNode`를 설정한다. 이것은 `A.next`를 `C`로 설정하는 것이다.
    - 아직 한 쌍의 노드가 남아있는 경우, 이 설정은 다음 스왑의 여러 단계에서 재정의된다.
    - 2단계에서는 `head.next.next`를 수행하므로, `head`와 `head.next`가 존재하는지 확인하기 위해 while 루프의 조건이 필요하다. 즉, 리스트에 노드가 하나만 남아 있으면 현재 반복이 끝난 후 while 루프가 종료된다. 따라서 이 설정은 재정의되지 않는다.
    - 예를 들어, `A -> B -> C -> D` 리스트에서 어느 시점에 `B <-> A -> C -> D`가 있을 수 있다. 여러 단계를 수행하면 `B -> A -> C -> D`가 된다. `C, D` 쌍을 바꾸기 시작하면, 여러 단계 중 하나에서 `D` 옆에 A를 설정하여 방금 수행한 작업을 재정의한다. 하지만 `D`가 존재하지 않는다면 반복은 그냥 끝난다. 이 시나리오에서는 `B -> A -> C`가 되는 것이 우리가 원하는 결과이다.

단계를 요약하면 다음과 같다:

1. `A -> B -> C -> ...` 에서 `A <-> B -> C -> ...` 로 엣지 스왑을 수행한다.
2. 현재 쌍을 제외한 나머지 리스트에 계속 액세스할 수 있는지 확인한다(`C` 저장).
3. 이제 `A <-> B`가 나머지 리스트에서 분리되었으므로, 나중에 나머지 리스트과 연결할 수 있도록 `A`에 대한 포인터를 저장한다. 다음 쌍으로 이동한다.
4. 이전 쌍을 나머지 리스트에 연결한다. 이 경우, `A -> D`를 연결한다.
5. 더미 포인터를 사용하여 반환하려는 항목에 대한 참조를 유지한다.
6. 노드 수가 홀수인 경우를 처리한다.

> 여기서 단계의 순서는 시간 순서가 아니다. 문제의 요구 사항을 고려할 때 생각할 수 있는 순서일 뿐이다.
{: .prompt-general }

```cpp
class Solution {
public:
    ListNode* swapPairs(ListNode* head) {
        // 경계 조건 검사: 링크드 리스트에 노드가 0개 또는 1개 있는 경우, 그대로 반환
        if (head == nullptr || head->next == nullptr) {
            return head;
        }
        
        ListNode* dummy = head->next;    // 단계 5
        ListNode* prev = nullptr;        // 단계 3을 위한 초기 설정
        while (head != nullptr && head->next != nullptr) {
            if (prev != nullptr) {
                prev->next = head->next; // 단계 4
            }
            prev = head;                 // 단계 3
            
            // 단계 2
            ListNode* nextNode = head->next->next;
            head->next->next = head;     // 단계 1
            
            head->next = nextNode;       // 단계 6
            head = nextNode;             // 다음 쌍으로 이동 (단계 7)
        }
        
        return dummy; // 처음 두 노드가 바뀐 상태의 링크드 리스트의 새로운 헤드를 반환
    }
};
```

이 알고리즘의 시간 복잡도는 $$O(n)$$이며, 여기서 n은 링크드 리스트의 노드 수다. while 루프는 $$n$$번 실행되며, 각 반복에서 수행되는 작업은 상수 시간, 즉 $$O(1)$$이 필요하다. 몇 개의 포인터만 사용하기 때문에 공간 복잡도는 $$O(1)$$이다.

링크드 리스트 문제를 해결할 때는 문제를 세분화하는 것이 중요하다. 달성해야 할 목표와 그 목표를 달성하기 위한 단계를 구체적으로 나열해야 한다. 이 예제에서 볼 수 있듯이, 생각하는 단계의 순서가 실제로 수행해야 하는 단계의 순서와 일치하지 않을 수도 있다.

LeetCode와 같은 플랫폼에서는 처음 시도할 때 몇 가지 단계를 놓칠 수 있으며, 잘못된 답안을 제출하면 피드백을 통해 이를 알 수 있다. 면접 상황에서는 자신의 사고 과정을 명확하게 전달해야 하며, 면접관은 누락된 부분을 지적해 줄 수 있다. 특히, 종이와 펜을 이용하여 `1 -> 2 -> 3 -> 4`와 같은 테스트 케이스를 작성하고 로직을 따라가 보는 것이 매우 유용하다.

---

## **리스트 반전은 알고리즘의 일부일 뿐이다**

리스트를 뒤집는 것은 "다른 문제를 해결하기 위한 단계가 될 수 있는 기술"이라고 설명했다. 이해를 돕기 위해 간단한 예를 들어보겠다.

[2130. 링크드 리스트의 최대 쌍 합계](https://leetcode.com/problems/maximum-twin-sum-of-a-linked-list/)문제는 리스트의 최대 쌍 합계를 찾는 것이다. 쌍은 첫 번째와 마지막 노드, 두 번째와 끝에서 두 번째 노드 등으로 구성된다.

하나의 간단한 해결책은 링크드 리스트를 배열로 변환하고 인덱싱을 통해 각 쌍에 쉽게 접근하는 것이다. 하지만 더 우아한 $$O(1)$$ 공간 해결책은 다음과 같다:

1. 빠른 포인터와 느린 포인터 기법을 사용하여 링크드 리스트의 중간을 찾는다.
2. 리스트의 중간부터 리스트를 뒤집는다. 이 방법은 기본적으로 리스트의 뒷부분만 뒤집는다.
3. 뒷부분을 뒤집은 후, 각 노드는 이제 쌍의 노드와 `n/2`의 거리에 있다. 여기서 n은 리스트의 전체 노드 수다.
4. 이 사실을 기반으로 `slow` 포인터보다 `n/2` 앞에 새로운 빠른 포인터를 위치시킨다. 이제 리스트의 시작부터 `n/2`번 반복하여 모든 쌍의 합계를 `slow.val + fast.val`로 계산할 수 있다.

> 실제로 이 알고리즘을 구현해보는 것이 좋은 연습이 될 것이다.
{: .prompt-general }

위의 예시와 달리, 리스트를 뒤집는 것은 문제 해결의 핵심이 아니라, 보다 효과적인 해결책에 도달하기 위한 수단일 뿐이다. 어떻게 빠른 포인터와 느린 포인터를 사용하는지 주목해보자.

---

링크드 리스트는 여기서 언급하지 않은 여러 용도가 있다.

예를 들어, 해싱을 다룰 때 충돌을 처리하기 위해 링크드 리스트를 사용하는 체이닝 기법이 있음을 언급했다. 또한 덱 같은 데이터 구조를 링크드 리스트로 구현할 수 있다는 것을 나중에 다루겠다. 이러한 접근 방식을 사용하여 연습 문제를 해결하고, 달성해야 할 작업과 그 단계를 세분화해 보는 것이 중요하다.

대부분의 링크드 리스트 문제는 특별한 기술을 요구하지 않으며, 포인터의 이동 방식을 이해하기만 하면 해결할 수 있다. 이 기술을 연마하는 가장 좋은 방법은 계속 연습하는 것이다.

---

## **보너스 문제**

### 빠른 포인터와 느린 포인터

- [2095. Delete the Middle Node of a Linked List](https://leetcode.com/problems/delete-the-middle-node-of-a-linked-list/)
- [19. Remove Nth Node From End of List](https://leetcode.com/problems/remove-nth-node-from-end-of-list/)
- [82. Remove Duplicates from Sorted List II](https://leetcode.com/problems/remove-duplicates-from-sorted-list-ii/)
- [1721. Swapping Nodes in a Linked List](https://leetcode.com/problems/swapping-nodes-in-a-linked-list/)

### 링크드 리스트 뒤집기

- [234. Palindrome Linked List](https://leetcode.com/problems/palindrome-linked-list/)
- [2074. Reverse Nodes in Even Length Groups](https://leetcode.com/problems/reverse-nodes-in-even-length-groups/)
- [2130. Maximum Twin Sum of a Linked List](https://leetcode.com/problems/maximum-twin-sum-of-a-linked-list/)

### 일반

- [203. Remove Linked List Elements](https://leetcode.com/problems/remove-linked-list-elements/)
- [1290. Convert Binary Number in a Linked List to Integer](https://leetcode.com/problems/convert-binary-number-in-a-linked-list-to-integer/)
- [328. Odd Even Linked List](https://leetcode.com/problems/odd-even-linked-list/)
- [707. Design Linked List](https://leetcode.com/problems/design-linked-list/)

---

출처: [Leetcode](https://leetcode.com/explore/interview/card/leetcodes-interview-crash-course-data-structures-and-algorithms/704/linked-lists/4600/)
