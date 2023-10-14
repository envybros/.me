---
title: "[배열 및 문자열] 자주 쓰는 패턴 및 트릭"
categories: [Algorithm 연구소]
tags: [Algorithm]
date: 2023-09-10 01:30
math: true
img_path: /assets/img/algorithm/
---

---

![Title](algorithm_title.png)

---

## **개요**

이 글에서는 배열과 문자열 관련 알고리즘 문제에서 활용 가능한 주요 패턴과 일반적인 트릭에 대해 설명할 것이다.

## **O(n) 문자열 구축**

앞서 언급했듯이 대부분의 프로그래밍 언어에서 문자열은 불변이다. 따라서 문자열에 하나의 문자를 추가하는 것은 $$O(n)$$의 연산이 필요하다. 예를 들면, 100만 글자의 문자열에 문자 하나를 추가하려면 그 100만 글자를 새로운 문자열로 복사해야 한다.

많은 문제에서는 알고리즘을 통해 문자열을 반환하게 된다. 만약 최종 문자열의 길이가 `n`이며 이를 한 번에 한 글자씩 만든다면, 시간 복잡도는 어떻게 될까? 각 단계에서 필요한 연산은 `1 + 2 + 3 + ... + n`이며, 이것은 $$O(n^2)$$의 시간 복잡도를 가진다.

문자열이 불변인 언어를 사용할 경우, 이와 같은 단순 연결 방식은 $$O(n^2)$$의 시간 복잡도를 초래한다.

$$O(n)$$ 시간 내에 문자열을 효율적으로 구축하는 방법도 있다. 이 방법은 사용하는 언어에 따라 다를 수 있다. 여기서는 Python과 Java의 방법을 설명하지만, 다른 언어를 사용하는 사람은 해당 언어에 맞는 방법을 찾아보면 좋다.

### **Python**

1. 리스트를 선언한다.
2. 문자열을 작성할 때 리스트에 문자를 추가한다. 이 연산은 $$O(1)$$의 시간 복잡도를 가진다. 따라서 n개의 연산은 $$O(n)$$의 시간이 소요된다.
3. 모든 문자가 추가되면 `"".join(list)`를 사용해 리스트를 문자열로 변환한다. 이 작업은 $$O(n)$$의 시간이 소요된다.
4. 따라서 총 연산 비용은 $$O(n + n)$$ = $$O(2n)$$이며, 이는 $$O(n)$$으로 간주된다.

```py
def build_string(s):
    arr = []
    for c in s:
        arr.append(c)

    return "".join(arr)
```

### **Java**

1. [StringBuilder](https://docs.oracle.com/javase/7/docs/api/java/lang/StringBuilder.html) 클래스를 사용한다.
2. 문자열을 만들 때 각 문자를 StringBuilder에 추가한다. 이 연산은 $$O(1)$$의 시간 복잡도를 가진다. 따라서 n번의 연산에는 $$O(n)$$의 시간이 소요된다.
3. 완료되면 StringBuilder.toString() 메서드를 사용하여 StringBuilder의 내용을 문자열로 변환한다. 이 연산은 $$O(n)$$의 시간 복잡도를 가진다.
4. 따라서 총 연산 비용은 $$O(n + n)$$ = $$O(2n)$$이며, 이는 $$O(n)$$으로 간주된다.

```java
public string buildString(String s) {
    StringBuilder sb = new StringBuilder();
    for (int i = 0; i < s.length(); i++) {
        sb.append(s.charAt(i));
    }

    return sb.toString();
}
```

> C++와 JavaScript에서는 문자열을 만들 때 `+=` 연산자만 사용해도 충분하다.
{: .prompt-general }

---

## **부분 배열/부분 문자열, 부분 수열 및 부분 집합**

부분 배열, 부분 문자열, 부분 수열, 부분 집합의 차이점과 그로 인한 문제 해결 시 주의해야 할 점을 설명할 것이다.

### **부분 배열 / 부분 문자열**

> 부분 배열 또는 부분 문자열은 배열 또는 문자열의 연속된 부분을 의미한다.
{: .prompt-general }

문제에서 다음과 같은 명시적인 제약 조건을 갖는 경우:

- 합계가 `k`보다 크거나 작다
- 최대 `k`개의 고유한 요소가 있거나 중복이 허용되지 않는 경우

또는 다음과 같은 내용을 요구하는 경우:

- 최소 또는 최대 길이
- 부분 배열이나 부분 문자열의 개수
- 최대 또는 최소 합계

슬라이딩 윈도우 방식을 고려해볼 수 있다. 하지만 모든 문제를 슬라이딩 윈도우로 해결해야 하는 것은 아니며, 모든 슬라이딩 윈도우 문제가 위와 같은 특성을 갖는 것도 아니다. 위의 특성은 일반적인 지침으로 생각하면 된다.

문제의 입력이 정수 배열이고 여러 부분 배열의 합을 계산해야 하는 경우, 누적 합을 사용하는 것이 좋다.

`i`와 `j` 사이의 부분 배열의 길이(포함)는 `j - i + 1`이다. 이는 `i` 이상에서 시작하여 `j`에서 끝나는 부분 배열의 길이이기도 하다.

---

## **부분 수열 (subsequence)**

> 부분 수열은 배열이나 문자열의 요소 집합으로, 동일한 상대 순서를 유지하지만 연속적일 필요는 없다.
>
> 예를 들어, `[1, 2, 3, 4]`의 부분 수열에는 `[1, 3], [4], [], [2, 3]`이 포함되지만, `[3, 2], [5], [4, 1]`은 포함되지 않는다.
{: .prompt-general }

일반적으로 부분 수열은 더 어렵다. 지금은 첫 번째 챕터이기 때문에 부분 수열 패턴에 대해 깊게 다루기는 어렵다. 예를 들어, 많은 부분 수열 문제를 푸는 데에는 동적 프로그래밍이 사용되므로, 부분 수열에 대해서는 나중에 다시 다룰 것이다.

현재까지 배운 패턴 중에서는, 두 개의 입력 배열 또는 문자열이 주어졌을 때 투 포인터를 사용하는 방식이 가장 흔하다. 하지만, 누적 합과 슬라이딩 윈도우는 부분 배열/문자열과 관련이 있기 때문에 이 경우에는 적용되지 않는다.

---

## **부분 집합 (Subsets)**

> 부분 집합은 원래의 배열이나 문자열에서 뽑아낸 모든 요소의 집합을 말한다. 순서는 중요하지 않고, 요소들이 연속적일 필요도 없다.
>
> 예를 들어, `[1, 2, 3, 4]`의 부분 집합에는 `[3, 2], [4, 1, 2], [1]`이 포함된다. 동일한 요소를 가진 부분 집합은 같다고 여기므로, `[1, 2, 4]`와 `[4, 1, 2]`는 같은 부분 집합이다.
{: .prompt-general }

부분 집합과 부분 수열의 차이는 무엇일까? 정수 배열에서 원소 3개가 연속적인 부분 수열을 찾는 것과 원소 3개를 포함하는 부분 집합을 찾는 것은 다르다. 부분 집합에서는 원소 3개가 어떤 순서로든 포함되기만 하면 되지만, 부분 수열에서는 원소들이 특정한 순서로 연속적으로 나타나야 한다.

아직 첫 번째 장이기 때문에 부분 집합에 대해 많이 다루기는 어렵다. 백트래킹 장에서 부분 집합을 활용하는 방법을 알게 될 것이다.

문제에서 부분 집합의 순서가 중요하지 않다면(예: 부분 집합의 합을 찾는 경우), 부분 집합을 그대로 사용할 수 있다. 부분 집합에 관련된 처리에서 특히 주의해야 할 점은 순서가 중요하지 않기 때문에 입력을 정렬해도 무방하다는 것이다.

---

## **문제 풀이**

### **투 포인터**

- 557. [Reverse Words in a String III](https://leetcode.com/problems/reverse-words-in-a-string-iii/)
- 917. [Reverse Only Letters](https://leetcode.com/problems/reverse-only-letters/)
- 283. [Move Zeroes](https://leetcode.com/problems/move-zeroes/)
- 2000. [Reverse Prefix of Word](https://leetcode.com/problems/reverse-prefix-of-word/)

### **슬라이딩 윈도우**

- 209. [Minimum Size Subarray Sum](https://leetcode.com/problems/minimum-size-subarray-sum/)
- 1456. [Maximum Number of Vowels in a Substring of Given Length](https://leetcode.com/problems/maximum-number-of-vowels-in-a-substring-of-given-length/)
- 1208. [Get Equal Substrings Within Budget](https://leetcode.com/problems/get-equal-substrings-within-budget/)

### **누적 합**

- 1732. [Find the Highest Altitude](https://leetcode.com/problems/find-the-highest-altitude/)
- 724. [Find Pivot Index](https://leetcode.com/problems/find-pivot-index/)
- 303. [Range Sum Query - Immutable](https://leetcode.com/problems/range-sum-query-immutable/)

---

출처: [Leetcode](https://leetcode.com/explore/interview/card/leetcodes-interview-crash-course-data-structures-and-algorithms/703/arraystrings/4504/)
