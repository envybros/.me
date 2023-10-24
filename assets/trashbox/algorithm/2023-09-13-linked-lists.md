---
title: "링크드 리스트 (Linked list)"
categories: [Algorithm 연구소]
tags: [Algorithm]
date: 2023-09-13 01:30
math: true
img_path: /assets/img/algorithm/
---

---

![Title](algorithm_title.png)

---

## **개요**

> 이 장을 시작하기 전에 클래스, 객체 및 속성을 비롯한 객체 지향 프로그래밍 개념에 대한 기본적인 이해가 필요하다.
{: .prompt-general }

**노드**의 개념을 소개하는 것으로 시작하겠다. 노드는 하나의 요소로 생각할 수 있지만, 정수나 문자열과 같은 단순한 데이터보다 더 많은 정보를 담고 있다. 노드는 추상적인 개념이다. 예를 들어 배열 `[1, 2, 3]`이 있다고 가정해 볼 것이다. 각 요소를 정수와 인덱스라는 두 가지 정보를 가진 노드라고 상상할 수 있다. 따라서 두 번째 요소는 다음과 같다.

```text
data: 2
index: 1
```

배열은 내부적으로 요소가 메모리에 연속적으로 저장되는 방식으로 구현된다. 32비트 정수를 저장하는 배열을 선언했다고 가정해 보자. 배열의 각 요소는 이웃 요소로부터 4바이트(32비트) 떨어진 주소에 있다. 이를 통해 프로그래머는 배열의 요소에 인덱싱(예: `arr[6]` 등)을 사용하여 액세스할 수 있다.

링크드 리스트는 배열과 유사한 데이터 구조이지만, 구현 방식에 차이가 있다. 링크드 리스트도 데이터를 순차적으로 저장하지만, 노드 객체를 사용하여 구현된다(노드 객체를 정의하는 사용자 정의 클래스가 필요하다). 각 노드에는 시퀀스의 다음 요소를 나타내는 노드를 가리키는 "next" 포인터가 있다.

![Linked List](LinkedList.png)

다음은 데이터 `1 --> 2 --> 3`을 나타내는 링크드 리스트를 만드는 예제 코드이다. 보다시피, 노드를 정의하는 클래스에는 데이터를 저장할 필드 `val`과 다음 노드를 참조하는 `next` 포인터가 있다. 코드에서는 각 숫자에 대해 하나씩 세 개의 노드를 생성한 다음 그에 따라 `next` 포인터를 설정하고 있다.

```cpp
struct LinkedListNode {
    int val;
    LinkedListNode *next;
    LinkedListNode(int val): val (val), next(nullptr) {}
};

int main() {
    LinkedListNode* one = new LinkedListNode(1);
    LinkedListNode* two = new LinkedListNode(2);
    LinkedListNode* three = new LinkedListNode(3);
    one->next = two;
    two->next = three;
    LinkedListNode* head = one;
    
    cout << head->val << endl;
    cout << head->next->val << endl;
    cout << head->next->next->val << endl;
}
```

`1`이 있는 노드는 링크드 리스트의 시작이므로 `head`라고 부른다. 일반적으로 헤드에 대한 참조를 유지하는 것이 중요하다. 헤드는 링크드 리스트의 모든 요소에 도달할 수 있는 시작점이기 때문에, 헤드를 참조하면 어떤 요소도 "잃어버리지" 않게 된다.

---

## **배열과의 장단점 비교**

알고리즘 문제에서 배열과 링크드 리스트의 장단점을 비교하는 것은 크게 중요하지 않다. 링크드 리스트를 사용하는 대부분의 문제는 링크드 리스트를 입력으로 받기 때문에, 이를 사용할지 말지를 선택할 수 없는 경우가 많다. 그렇지만 최적의 알고리즘을 위해 링크드 리스트를 활용하는 문제들도 있으며, 면접 상황에서 이에 대한 질문을 받을 수도 있기 때문에 이들의 장단점을 이해해두는 것이 도움이 될 수 있다.

링크드 리스트의 가장 큰 장점은 $$O(1)$$ 시간 복잡도로 어느 위치에서든 요소를 추가하거나 제거할 수 있다는 점이다. 하지만 이를 위해서는 해당 위치의 노드에 대한 참조를 알고 있어야 하며, 그렇지 않다면 `head`부터 해당 위치까지 탐색해야 하므로 $$O(n)$$ 시간이 소요된다. 이 점은 (동적)배열이 요소의 추가 및 제거에 $$O(n)$$ 시간이 필요한 것과 비교했을 때 링크드 리스트의 유리한 점으로 볼 수 있다.

반면, 링크드 리스트의 큰 단점은 임의 접근(Random Access)이 불가능하다는 것이다. 예를 들어, 링크드 리스트에서 150,000번째 요소에 접근하고자 한다면, 대개는 처음부터 150,000번 반복하여 탐색해야 한다. 이는 배열이 $$O(1)$$ 시간 복잡도로 인덱싱을 제공하는 것과 대조적이다. 링크드 리스트는 특정 위치의 요소에 접근하기 위해 $$O(n)$$ 시간이 걸릴 수 있다.

알고리즘 문제와는 직접적인 관련이 적지만, 면접에서 이야기할 수 있는 추가적인 내용으로 링크드 리스트의 또 다른 장점은 그 크기가 고정되어 있지 않다는 것이다. 동적 배열은 크기 조정이 가능하긴 하지만, 내부적으로는 일정한 크기가 할당되어 있다. 이 크기를 초과하는 경우 배열이 리사이징되며 이 과정에서 추가적인 비용이 발생한다. 링크드 리스트는 이런 제약이 없다. 하지만 링크드 리스트는 각 요소가 포인터를 저장하기 위한 추가적인 공간을 필요로 하기 때문에, 배열에 비해 메모리 오버헤드가 더 크다. 특히 부울 값이나 문자와 같이 크기가 작은 데이터를 저장하는 경우에는 필요한 공간이 배열보다 훨씬 많아질 수 있다.

---

## **링크드 리스트의 메커니즘**

링크드 리스트의 노드와 포인터를 코드를 통해 조작하는 방법을 이해하는 것은 링크드 리스트 관련 면접 문제를 해결하는 데 필수적이며, 포인터 처리의 기본 개념은 모든 소프트웨어 엔지니어에게 필수적인 기술이다.

### **할당(=)**

링크드 리스트의 기존 노드에 포인터를 할당하면, 그 포인터는 메모리 상의 객체를 가리키게 된다. 예를 들어 노드의 `head`가 있다고 가정해 보자:

```cpp
ListNode* ptr = head;
head = head->next;
head = nullptr;
```

> C++과 같은 언어에서는 별표(`*`)로 표시되는 명시적 포인터를 사용한다. 명시적 포인터가 없는 언어에서는, 모든 원시 타입이 아닌 변수(예: 사용자 정의 클래스 객체)가 기본적으로 포인터로 취급된다.
{: .prompt-general }

위 코드를 실행한 후, `head` 변수가 변경되어도 `ptr` 변수는 원래 `head`가 가리켰던 노드를 계속 가리킨다. 이는 포인터의 첫 번째 중요한 개념으로, `ptr`을 직접 변경하지 않는 한(`ptr = something` 등의 할당이 없는 한) `ptr`이 가리키는 노드는 변하지 않는다.

### **.next 체인 연결**

여러 개의 `.next` 연산이 연결된 경우, 마지막 `.next`를 제외한 모든 `.next` 연산은 특정 노드를 가리킨다. 예를 들어, `1 -> 2 -> 3`의 링크드 리스트에서 `head`가 첫 번째 노드를 가리키고 있을 때, `head.next.next`는 세 번째 노드가 아닌 두 번째 노드의 `next`을 가리키므로, 이는 세 번째 노드를 가리키게 된다. 이러한 접근 방식은 특히 링크드 리스트를 다룰 때 유용하다.

### **순회**

링크드 리스트를 순회하는 것은 간단한 루프를 통해 가능하다. 다음은 링크드 리스트의 모든 노드의 값을 합산하는 예제 코드이다.

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

> 링크드 리스트의 마지막 노드의 `next` 포인터는 `null`이다. 따라서, 마지막 노드에서 `head = head->next`를 수행하면 `head`는 `null`이 되어 while 루프가 종료된다.
{: .prompt-general }

`head->next`로 이동하는 것은 배열에서 다음 요소로 넘어가는 것과 유사하다. 또한, 재귀적 방법을 사용하여 리스트를 순회할 수도 있다.

```cpp
int getSum(ListNode* head) {
    if (head == nullptr) {
        return 0;
    }

    return head->val + getSum(head->next);
}
```

이 코드는 현재 노드의 값을 다음 노드의 값과 더하는 재귀 함수로, 노드가 null일 때까지 계속된다.

---

## **링크드 리스트의 유형**

### **싱글 링크드 리스트**

싱글 링크드 리스트는 가장 흔한 링크드 리스트의 형태로, 각 노드는 다음 노드를 가리키는 포인터 하나만을 가진다. 이 때문에, 리스트를 순회할 때는 앞쪽 방향으로만 이동이 가능하다. 다음 노드를 가리키는 포인터는 일반적으로 `next`라고 명명된다.

리스트의 특정 위치, 즉 `i`번째 위치에 요소를 추가하고자 한다고 가정해보자. 이를 위해서는 먼저 `i - 1`번째 위치에 있는 요소에 접근할 수 있어야 한다. 추가하려는 요소의 `next`는 `i`번째 요소를 가리키게 되고, `i - 1`번째 요소의 `next`는 새로운 요소를 가리키도록 변경되어야 한다. 아래 코드와 설명은 이러한 과정을 보여준다:

```cpp
struct ListNode {
    int val;
    ListNode *next;
    ListNode(int val) : val(val), next(nullptr) {}
};

// prevNode는 i - 1번째 위치의 노드를 가리킨다
void addNode(ListNode* prevNode, ListNode* nodeToAdd) {
    nodeToAdd->next = prevNode->next;
    prevNode->next = nodeToAdd;
}
```

> 참고: 이 연산을 수행하기 위해서는 해당 위치의 이전 노드에 대한 참조가 필요하지만, 일반적으로 이러한 연산은 리스트를 순회하며 즉석에서 수행된다. 원하는 위치의 참조가 없는 경우, 연산의 시간 복잡도는 $$O(n)$$이 되며, 참조가 이미 있는 경우에는 $$O(1)$$이 된다.
{: .prompt-general }

위치 `i`에 있는 요소를 삭제하는 상황을 생각해보자. 이 경우 역시 `i - 1`번째 요소에 접근할 수 있어야 한다. 삭제 연산 후, `i + 1`번째 요소는 `i`번째 위치로 이동해야 한다. 따라서 `i - 1`번째 요소의 `next`는 `i + 1`번째 요소를 가리키도록 업데이트해야 한다. 아래 코드는 이 과정을 설명한다:

```cpp
struct ListNode {
    int val;
    ListNode *next;
    ListNode(int val) : val(val), next(nullptr) {}
};

// prevNode는 i - 1번째 위치의 노드를 가리킨다
void deleteNode(ListNode* prevNode) {
    prevNode->next = prevNode->next->next;
}
```

> 노드를 삭제할 때, `prevNode.next`는 삭제될 노드를 가리킨다. 이 연결을 끊은 후 `prevNode.next`는 삭제되어야 할 노드의 다음 노드, 즉 `prevNode.next.next`를 가리키도록 변경된다.
{: .prompt-general }

이러한 삽입 및 삭제 연산의 시간 복잡도는 이전 노드에 대한 참조가 있을 경우 $$O(1)$$이지만, 없을 경우 전체 리스트를 순회해야 하므로 $$O(n)$$이 된다.

---

### **더블 링크드 리스트**

더블 링크드 리스트는 싱글 링크드 리스트와 유사하지만, 각 노드는 이전 노드를 가리키는 추가적인 포인터를 가진다. 이 포인터는 `prev`라고 명명되며, 리스트를 양방향으로 순회할 수 있게 한다.

싱글 링크드 리스트에서는 특정 위치의 요소를 추가하거나 삭제하려면 해당 위치의 이전 노드, 즉 `prev` 노드에 접근해야 했다. 하지만 더블 링크드 리스트에서는 해당 위치의 노드 자체에 접근하면, `prev` 포인터를 사용하여 바로 이전 노드를 찾을 수 있으므로 연산이 간단해진다.

더블 링크드 리스트에서 노드를 추가하거나 삭제할 때는 `prev` 포인터를 업데이트하는 추가 작업이 필요하다.

```cpp
struct ListNode {
    int val;
    ListNode *next;
    ListNode *prev;
    ListNode(int val) : val(val), next(nullptr), prev(nullptr) {}
};

// node는 i번째 위치의 노드를 가리킨다
void addNode(ListNode* node, ListNode* nodeToAdd) {
    ListNode* prevNode = node->prev;
    nodeToAdd->next = node;
    nodeToAdd->prev = prevNode;
    prevNode->next = nodeToAdd;
    node->prev = nodeToAdd;
}

void deleteNode(ListNode* node) {
    ListNode* prevNode = node->prev;
    ListNode* nextNode = node->next;
    prevNode->next = nextNode;
    nextNode->prev = prevNode;
}
```

더블 링크드 리스트를 사용하면 각 노드가 이전 노드와 다음 노드를 모두 알고 있기 때문에, 싱글 링크드 리스트보다 더 다양한 유형의 연산을 효과적으로 처리할 수 있다.

---

## **센티널 노드와 링크드 리스트**

> 링크드 리스트의 시작 부분을 `헤드(head)`, 끝 부분을 `테일(tail)`이라고 한다.
{: .prompt-general }

센티널 노드는 링크드 리스트의 시작과 끝에 위치하며, 연산과 해당 연산을 실행하는 데 필요한 코드를 더 깔끔하게 만드는 데 사용된다. 링크드 리스트에 노드가 없는 경우에도 `head`와 `tail`에 대한 포인터를 유지한다는 개념이다. 링크드 리스트의 실제 `head`는 head.next이고 실제 `tail`은 `tail.prev`이다. 센티널 노드 자체는 링크드 리스트의 데이터를 담고 있지 않지만 리스트의 일부로 간주될 수 있다.

> 앞서 살펴본 코드는 오류가 발생하기 쉽다. 예를 들어, 목록의 마지막 노드를 삭제하려고 하면 `nextNode`가 `null`이 되고 `nextNode.next`에 액세스하려고 하면 오류가 발생한다. 센티널 노드를 사용하면 마지막 노드의 다음이 센티널 `tail`을 가리키기 때문에 이러한 시나리오에 대해 걱정할 필요가 없다.
{: .prompt-general }

또한 센티널 노드를 사용하면 링크드 리스트의 앞이나 뒤에서 쉽게 추가하고 제거할 수 있다. 연산을 수행하려는 위치의 노드에 대한 참조가 있는 경우 추가 및 제거는 $$O(1)$$에 불과하다는 점을 기억해보자. 센티널 `tail` 노드를 사용하면 $$O(1)$$의 목록 끝에서 연산을 수행할 수 있다.

```cpp
struct ListNode {
    int val;
    ListNode *next;
    ListNode *prev;
    ListNode(int val) : val(val), next(nullptr), prev(nullptr) {}
};

void addToEnd(ListNode* nodeToAdd) {
    nodeToAdd->next = tail;
    nodeToAdd->prev = tail->prev;
    tail->prev->next = nodeToAdd;
    tail->prev = nodeToAdd;
}

void removeFromEnd() {
    if (head->next == tail) {
        return;
    }

    ListNode* nodeToRemove = tail->prev;
    nodeToRemove->prev->next = tail;
    tail->prev = nodeToRemove->prev;
}

void addToStart(ListNode* nodeToAdd) {
    nodeToAdd->prev = head;
    nodeToAdd->next = head->next;
    head->next->prev = nodeToAdd;
    head->next = nodeToAdd;
}

void removeFromStart() {
    if (head->next == tail) {
        return;
    }

    ListNode* nodeToRemove = head->next;
    nodeToRemove->next->prev = head;
    head->next = nodeToRemove->next;
}

ListNode* head = new ListNode(-1);
ListNode* tail = new ListNode(-1);
head->next = tail;
tail->prev = head;
```

---

## **더미 포인터**

앞서 언급했듯이, 일반적으로 모든 요소에 항상 액세스할 수 있도록 `head`를 참조로 유지하려고 한다. 때로는 "더미" 포인터를 사용하여 순회하고 `head`를 유지하는 것이 더 좋다.

```cpp
int getSum(ListNode* head) {
    int ans = 0;
    ListNode* dummy = head;
    while (dummy != nullptr) {
        ans += dummy->val;
        dummy = dummy->next;
    }

    // 이전과 동일하지만, head에 대한 포인터를 계속 유지함
    return ans;
}
```

`dummy` 포인터를 사용하면 `head`에 대한 참조를 잃지 않고 링크드 리스트를 순회할 수 있다.

---

링크드 리스트의 대부분은 포인터를 이동하는 것이다. 포인터를 올바르게 재할당하는 방법과 재할당 순서에 대한 직관력을 키우는 것이 중요하다. 이를 위한 가장 좋은 방법은 프로세스를 시각적으로 확인하고 예제를 통해 작업하는 것이다. 위 섹션의 코드를 읽고 추가 및 제거가 어떻게 처리되는지 확실히 이해할 수 있을 때까지 연습해보자.

링크드 리스트 문제의 경우, 그 복잡성 때문에 많은 해결책을 찾기가 어렵다. 많은 경우 링크드 리스트를 반복해서 모든 요소를 배열에 넣은 다음 배열을 사용하여 문제를 해결할 수 있다. 하지만 이렇게 하면 문제의 핵심을 놓치게 되고 면접에서 불합격할 가능성이 높다. 이 문제의 핵심은 일반적으로 $$O(1)$$ 공간을 사용하여 포인터를 깔끔하게 조작하는 것이다.

다음 몇 개의 글에서는 링크된 목록 문제와 일반적인 패턴의 예시를 살펴볼 것이다.

---

출처: [Leetcode](https://leetcode.com/explore/interview/card/leetcodes-interview-crash-course-data-structures-and-algorithms/704/linked-lists/4506/)
