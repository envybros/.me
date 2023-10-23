---
title: "[DSA] 재귀 함수"
categories: [Algorithm 연구소]
tags: [Algorithm, DSA, Recursion]
date: 2023-09-02 02:10
math: true
img_path: /assets/img/algorithm/
---

---

![Title](algorithm_title.png)

---

## **개요**

재귀는 문제를 해결하는 방법이다. 코드에서 재귀는 자기 자신을 호출하는 함수를 사용하여 구현된다.

재귀 알고리즘의 반대는 반복 알고리즘이다. 어떤 반복 알고리즘도 재귀적으로 작성될 수 있다는 것을 증명하는 [연구 분야](https://en.wikipedia.org/wiki/Computability_theory)가 있다. 반복 알고리즘은 반복을 시뮬레이션하기 위해 for 문과 while 문을 사용하는 반면, 재귀 알고리즘은 동일한 논리를 시뮬레이션하기 위해 함수 호출을 사용한다.

1부터 10까지의 숫자를 출력하고 싶다고 가정해보자. 다음은 반복 알고리즘을 위한 의사 코드이다:

```text
for (int i = 1; i <= 10; i++) {
    print(i)
}
```

여기에 동등한 재귀 알고리즘을 위한 의사 코드가 있다:

```text
function fn(i):
    print(i)
    fn(i + 1)
    return

fn(1)
```

각각의 `fn` 호출은 먼저 `i`를 출력한다(1부터 시작), 그리고 `i`를 증가시킨 상태로 `fn`을 다시 호출한다(다음 숫자를 출력하기 위해).

> 첫 번째 함수 호출은 `1`을 출력한 다음, `fn(2)`를 호출한다. `fn(2)`에서는 `2`를 출력한 다음, `fn(3)`을 호출한다. 이 과정이 계속된다.
{: .prompt-tip }

하지만 이 코드는 사실 어딘가 잘못되었다. 혹시 문제점이 보이는가? 저 코드는 함수 호출이 절대 멈추지 않을 것이다! 이 코드를 실행하면 자연수(양의 정수)가 무한히 출력될 것이다(또는 컴퓨터가 폭발할 때까지). 위 코드에서는 `fn(i + 1)`이 return 줄보다 앞에 오기 때문에 `return` 줄에는 절대 도달하지 않는다.

우리는 재귀가 멈추게 하기 위한 **종료 조건**이 필요하다. 종료 조건은 재귀 함수의 시작 부분에 있는 호출을 중단하는 조건이다.

```text
function fn(i):
    if i > 10:
        return

    print(i)
    fn(i + 1)
    return

fn(1)
```

> `fn(10)`을 호출한 후, 우리는 `10`을 출력하고 `fn(11)`을 호출한다. `fn(11)` 호출에서 우리는 종료 조건을 충족시켜 반환한다. 이제 우리는 `fn(10)` 호출로 돌아가서 다음 줄, 즉 return 문으로 이동한다. 이것은 우리가 `fn(9)` 호출로 돌아가게 하고, 이 과정이 `fn(1)` 호출에서 반환할 때까지 계속된다.
{: .prompt-tip }

재귀에 대해 이해해야 할 중요한 사항은 코드가 실행되는 순서, 즉 컴퓨터가 명령어를 실행하는 순서이다. 반복 프로그램의 경우 쉽다 - 맨 위에서 시작하여 줄 별로 진행한다. 재귀의 경우, 호출이 서로 위에 쌓일 수 있기 때문에 혼란스러울 수 있다. 다시 숫자를 출력하되, 이번에는 3까지만 출력해보자. 또 다른 print 문을 추가하고 줄 번호도 매겨보자:

```text
function fn(i):
1.  if i > 3:
2.    return

3.  print(i)
4.  fn(i + 1)
5.  print(f"i = {i}인 호출 종료")
6.  return

fn(1)
```

이 코드를 실행하면 출력에서 다음을 볼 수 있다:

```cpp
// 1
// 2
// 3
// i = 3인 호출 종료
// i = 2인 호출 종료
// i = 1인 호출 종료
```

보다시피, 텍스트를 출력하는 줄은 역순으로 실행된다. 원래의 `fn(1)` 호출은 먼저 `1`을 출력한 다음, `fn(2)`로 이동하여 `2`를 출력하고, 그 다음 `fn(3)`으로 이동하여 `3`을 출력한 다음, `fn(4)`로 이동한다. **이제 중요한 부분**은 재귀가 어떻게 "위로" "돌아가는지이다. `fn(4)`는 종료 조건을 충족시켜 반환한다. 이제 우리는 `i = 3`인 함수 호출로 돌아가고 **줄 4**가 끝났으므로 **줄 5**로 이동하여 `"i = 3인 호출 종료"`를 출력한다. 그 줄이 실행되면, 우리는 다음 줄로 이동하는데, 거긴 `return`이다. 이제 우리는 `i = 2`인 함수 호출로 돌아가고 **줄 4**가 끝났으므로 다시 다음 줄로 이동하여 `"i = 2인 호출 종료"`를 출력한다. 이것은 원래의 `fn(1)` 호출이 반환될 때까지 반복된다.

> 모든 함수 호출은 반환될 때까지 "존재"한다. 다른 함수 호출로 이동할 때 이전 함수 호출은 새로운 함수 호출이 반환될 때까지 대기한다. 호출 순서는 기억되며, 함수 내의 줄은 순서대로 실행된다.
>
> 각 함수 호출은 자체적인 로컬 스코프를 가지고 있다는 점에 유의해야 한다. 위의 예에서, 우리가 `f(3)`을 호출할 때, `i`의 3개의 "버전"이 동시에 존재한다. 첫 번째 호출에는 `i = 1`이 있고, 두 번째 호출에는 `i = 2`가 있으며, 세 번째 호출에는 `i = 3`이 있다. 만약 우리가 `f(3)` 호출에서 `i += 1`을 수행한다면, 그때 `i`는 `4`가 되지만, **오직** `f(3)` 호출에서만 그렇다. 다른 두 "버전"의 `i`는 서로 다른 스코프에 있기 때문에 영향을 받지 않는다.
{: .prompt-tip }

---

## **문제 분석하기**

위의 출력 예제는 상당히 무의미하다. 단순히 숫자를 출력하고 싶다면 for 반복문을 사용하는 것이 더 쉽다. 재귀가 빛을 발하는 곳은 문제를 "하위 문제"로 분해하고, 그 해결책을 결합하여 원래 문제를 해결할 때이다.

[피보나치 수](https://en.wikipedia.org/wiki/Fibonacci_number)에 대해 살펴보자. 피보나치 수는 `0, 1`로 시작하는 숫자 시퀀스다. 그 다음 각 숫자는 이전 두 숫자의 합으로 정의된다. 처음 몇 개의 피보나치 수는 `0, 1, 1, 2, 3, 5, 8`이다. 더 형식적으로는 다음과 같이 표현할 수 있다.

$F{n}$ = $F_{n - 1}$ + $F_{n - 2}$

이것을 재귀 관계 혹은 **점화식**이라고 부른다 - 항목들을 서로 연결하는 방정식이다.

피보나치 수열의 $n$번째 수를 반환하는 함수 `F(n)`을 의사코드로 작성해보자. 재귀 함수와 함께 종료 조건을 잊지 말아야 한다. 이 경우 종료 조건은 명시적으로 정의되어 있다: `F(0) = 0`과 `F(1) = 1`.

```text
function F(n):
    if n <= 1:
        return n

    oneBack = F(n - 1)
    twoBack = F(n - 2)
    return oneBack + twoBack
```

`F(3)`을 찾고 싶다고 가정해보자. `F(3)`을 호출하면 각 들여쓰기 수준이 함수 호출의 범위를 나타내며, 다음과 같은 흐름을 볼 수 있다:

```text
oneBack = F(2)
    oneBack = F(1)
        F(1) = 1
    twoBack = F(0)
        F(0) = 0
    F(2) = oneBack + twoBack = 1
twoBack = F(1)
    F(1) = 1
F(3) = oneBack + twoBack = 2
```

보다시피, 원래 문제 `F(3)`을 두 개의 더 작은 하위 문제, 즉 `F(2)`와 `F(1)`로 분해했다. 점화식과 종료 조건을 결합함으로써, 우리는 하위 문제를 해결하고 그 해결책을 사용하여 원래 문제를 해결할 수 있게 되었다.

이것이 재귀의 가장 일반적인 사용 방법이다 - 재귀 함수가 **주어진 입력에 대해 해결하려는 문제의 답을 반환하도록 한다**. 이 예에서, 우리가 주어진 입력에 대해 해결하려는 문제는 "$n$번째 피보나치 수는 무엇인가?"이다. 따라서, 우리는 함수가 입력 $n$에 따라 피보나치 수를 반환하도록 설계했다. 종료 조건과 점화식을 결정함으로써, 우리는 함수를 쉽게 구현할 수 있다.

이 아이디어를 따르면, 하위 문제를 해결하는 것은 쉽다 - 100번째 피보나치 수를 원한다면, 그것이 99번째와 98번째 피보나치 수의 합임을 정의에 의해 알고 있다. `F(100)`에 대한 함수 호출에서, `F(99)`와 `F(98)`을 호출하면 그 숫자들을 얻을 것임을 알 수 있다.

---

> 출처: [Leetcode](https://leetcode.com/explore/interview/card/leetcodes-interview-crash-course-data-structures-and-algorithms/715/introduction/4655/)
{: .prompt-info }