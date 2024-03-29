---
title: "문자열 문제"
categories: [Algorithm 연구소]
tags: [Algorithm]
date: 2023-09-15 02:30
math: true
img_path: /assets/img/algorithm/
---

---

![Title](algorithm_title.png)

---

## **개요**

스택과 관련된 문자열 문제가 자주 출제된다. 스택을 활용할 수 있는 문자열 문제의 일반적인 패턴은 문자열을 순회하면서 각 문자를 스택에 넣고, 반복하는 동안 스택의 최상단 문자를 현재 문자와 비교하는 것이다. 스택은 이전 문자들의 "기록"을 보관하기 때문에 문자열 내의 문자 일치 여부를 확인하는 데 유용하다. 여기에 몇 가지 예시를 들어 설명하겠다.

---

> **예제 1**: [유효한 괄호](https://leetcode.com/problems/valid-parentheses/)
>
> `'('`, `')'`, `'{'`, `'}'`, `'['` 및 `']'` 문자만 포함된 문자열 `s`가 주어질 때, 입력 문자열이 유효한지 판단해야 한다. 문자열이 유효하려면 모든 여는 괄호는 올바른 순서로 같은 유형의 닫는 괄호로 닫혀야 하고, 각 닫는 괄호는 정확히 하나의 여는 괄호를 닫아야 한다.
>
> 예를 들어, `s = "({})"` 및 `s = "(){}[]"`는 유효하지만, `s = "(]"` 및 `s = "({)}"`는 유효하지 않다.
{: .prompt-general }

"올바른" 순서는 최근에 여는 괄호의 종류에 따라 결정된다. 닫는 괄호가 나타날 때마다 가장 최근의 여는 괄호와 일치해야 한다. 예를 들어, 문자열이 `"({["`로 시작하고 다음 세 문자가 모두 닫는 괄호인 경우, 닫는 괄호들은 가장 최근의 여는 괄호 순서대로 나와야 한다: `"]})"` (그렇지 않으면 `"[)"` 와 같은 결과가 발생한다). 이 순서는 가장 마지막에 본 여는 괄호가 가장 먼저 닫혀야 하는 괄호이므로 스택의 선입후출(LIFO) 특성을 따른다.

문자열을 순회할 때마다 여는 괄호를 만나면 스택에 추가해야 한다. 닫는 괄호를 만나면 스택의 맨 위에서 가장 최근에 열렸던 여는 괄호를 꺼내서 비교해야 한다. 일치하면 계속 진행하고, 일치하지 않거나 스택에 열린 괄호가 없으면 (예: `"{}]"`) 문자열이 유효하지 않다고 판단할 수 있다. 마지막으로, 문자열이 유효하려면 스택이 비어 있어야 하며, 이는 모든 괄호가 올바르게 닫혔음을 의미한다 (예: `"(){"`).

여는 괄호와 닫는 괄호를 어떻게 연결할 수 있을까? 해시 맵을 사용하여 각 여는 괄호를 해당하는 닫는 괄호와 연결할 수 있다. 그런 다음 닫는 괄호를 보았을 때 스택의 맨 위 요소를 키로 사용하여 그 값이 현재 문자와 일치하는지 확인할 수 있다.

{% raw %}

```cpp
class Solution {
public:
    bool isValid(string s) {
        stack<char> stack;
        unordered_map<char, char> matching {{'(',')'}, {'{','}'}, {'[',']'}};
        
        for (char c: s) {
            if (matching.find(c) != matching.end()) {
                stack.push(c);  // 여는 괄호를 스택에 추가한다.
            } else {
                if (stack.empty()) {
                    return false;  // 스택이 비어 있으면 유효하지 않다.
                }
                
                char previousOpening = stack.top();  // 이전 여는 괄호를 가져온다.
                if (matching[previousOpening] != c) {
                    return false;  // 괄호가 일치하지 않으면 유효하지 않다.
                }
                
                stack.pop();  // 스택의 맨 위 요소를 제거한다.
            }
        }
        
        return stack.empty();  // 유효한 경우, 스택은 비어 있어야 한다.
    }
};
```

{% endraw %}

스택의 push 및 pop 연산은 $$O(1)$$의 시간 복잡도를 가지므로, 전체 알고리즘의 시간 복잡도는 $$O(n)$$이다. 여기서 n은 입력 문자열의 길이이다. 이는 문자열의 모든 문자를 한 번씩만 처리하기 때문이다. 스택의 크기가 입력 문자열의 길이에 비례하여 증가할 수 있기 때문에 공간 복잡도 역시 $$O(n)$$이다.

이 문제에서 스택을 사용하는 해결책의 핵심은 문제가 스택의 LIFO(선입후출) 특성, 즉 가장 최근에 삽입된 여는 괄호가 가장 먼저 제거되어야 함을 요구한다는 것을 인식하는 것이다.

---

> **예제 2**: [문자열에서 인접한 모든 중복 제거](https://leetcode.com/problems/remove-all-adjacent-duplicates-in-string/)
>
> 문자열 `s`가 주어지면 더 이상 제거할 수 없을 때까지 중복(같은 문자 두 개가 나란히 있는 경우)을 계속 제거한 후 최종 문자열을 반환한다.
>
> 예를 들어, `s = "abbaca"`인 경우 먼저 `"bb"`를 제거하여 `"aaca"`를 얻을 수 있다. 그런 다음 `"aa"`를 제거하여 `"ca"`를 얻을 수 있다. 이것이 최종 결과이다.
{: .prompt-general }

이 문제에서 어려운 점은 처음부터 모든 중복을 제거할 수 있는 것이 아니라는 것이다. 예를 들어, `"aa"`를 제거하기 위해서는 먼저 `"bb"`를 제거해야 한다. 만약 입력이 `s = "abccba"`라면, 삭제 순서는 `c -> b -> a`이다. 이러한 패턴은 마지막(가장 최근) 문자가 가장 먼저 삭제되는 LIFO 패턴을 따르며, 스택을 이용할 수 있다. 입력을 반복하면서 스택에 문자를 넣는다. 각 단계에서 스택의 맨 위 문자가 현재 문자와 같으면 두 문자가 인접하여 삭제할 수 있다.

```cpp
class Solution {
public:
    string removeDuplicates(string s) {
        // C++에서 문자열은 변경 가능하므로, 답을 직접 스택으로 사용
        string ans = "";
        for (char c: s) {
            if (!ans.empty() && ans.back() == c) {
                ans.pop_back();  // 중복을 찾으면 pop을 사용해 제거
            } else {
                ans.push_back(c);  // 중복이 아니면 문자 추가
            }
        }
        
        return ans;  // 최종 문자열 반환
    }
};
```

스택은 데이터를 최상단에서만 추가하고 제거하는 방식으로 작동하는 인터페이스로 정의된다. C++의 문자열은 변경이 가능하므로 문자열 자체를 스택으로 사용하고 결과를 직접 반환할 수 있다. 이 알고리즘의 시간 및 공간 복잡도는 $$O(n)$$이며, 여기서 $$n$$은 입력의 길이이다.

---

> **예제 3**: [백스페이스 문자열 비교](https://leetcode.com/problems/backspace-string-compare/)
>
> 두 문자열 `s`와 `t`가 주어졌을 때, 두 문자열을 비어 있는 텍스트 편집기에 입력했다고 가정할 때, 두 문자열이 동일하면 true를 반환한다. `'#'`은 백스페이스 문자를 의미한다.
>
> 예를 들어, `s = "ab#c"`, `t = "ad#c"`가 주어지면 true를 반환한다. 백스페이스가 적용되어 두 문자열은 모두 `"ac"`가 된다.
{: .prompt-general }

백스페이스는 최근에 입력한 문자를 삭제하는 LIFO 패턴을 따르므로, 스택을 사용하여 문자열 입력을 시뮬레이션하고 마지막에 문자열을 비교할 수 있다.

```cpp
class Solution {
public:
    bool backspaceCompare(string s, string t) {
        return build(s) == build(t);  // 두 문자열을 생성한 후 비교
    }
    
    string build(string s) {
        string ans = "";
        for (char c: s) {
            if (c != '#') {
                ans.push_back(c);  // 백스페이스가 아니면 문자 추가
            } else if (!ans.empty()) {
                ans.pop_back();  // 백스페이스면 가장 최근 문자 삭제
            }
        }
        
        return ans;  // 생성된 문자열 반환
    }
};
```

이 방법도 스택 구현이 효율적이므로 입력 크기에 따라 시간과 공간 복잡도가 선형적으로 증가한다.

---

출처: [Leetcode](https://leetcode.com/explore/interview/card/leetcodes-interview-crash-course-data-structures-and-algorithms/706/stacks-and-queues/4646/)
