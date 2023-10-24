---
title: "이진 탐색 (Binary Search)"
categories: [Algorithm 연구소]
tags: [Algorithm]
date: 2023-09-30 01:30
math: true
img_path: /assets/img/algorithm/
---

---

![Title](algorithm_title.png)

---

## **개요**

이진 탐색은 최악의 경우 $$O(\log{}n)$$ 시간 복잡도를 가지는 탐색 알고리즘이다. 여기서 n은 탐색 공간의 크기다. 이진 탐색을 사용하려면 탐색 공간이 정렬되어 있어야 한다. 우리가 트리와 그래프 장에서 살펴본 이진 탐색 트리는 이진 탐색에 기반한다.

보통 이진 탐색은 정렬된 요소의 배열에서 수행하지만, 다가오는 글에서 우리는 더 창의적인 방법으로 이진 탐색을 사용하는 법을 알아볼 것이다. 지금은 숫자로 정렬된 배열에 대해서만 이야기할 것이다.

정렬된 배열 `arr`와 원소 `x`가 있을 때, $$O(\log{}n)$$의 시간과 $$O(1)$$의 공간을 사용해, 이진 탐색은 다음을 수행할 수 있다:

1. `x`가 `arr`에 있으면 `x`의 인덱스를 찾는다.
2. 정렬된 상태를 유지하기 위해 `x`가 삽입될 수 있는 첫 번째 또는 마지막 인덱스를 찾는다.

이진 탐색의 아이디어는 다음과 같다:

정렬된 정수 배열 `arr`이 있고, 숫자 x가 그 안에 있다는 것을 알고 있지만, 어떤 인덱스에 있는지 모른다고 가정하자. `x`의 위치를 찾고 싶다. `arr`의 중간에 있는 요소를 확인함으로써 시작한다. 이 요소가 너무 작으면, 배열이 정렬되어 있으므로 왼쪽 절반의 모든 요소도 너무 작다는 것을 알 수 있다. 마찬가지로, 요소가 너무 크면 오른쪽 절반의 모든 요소도 너무 클 것이다.

`x`를 포함할 수 없는 절반을 버릴 수 있고, 다른 절반에 대해서 과정을 반복한다. `x`를 찾을 때까지 배열을 절반으로 계속 자른다.

이진 탐색이 구현되는 방식은 다음과 같다:

1. `left = 0`과 `right = arr.length - 1`을 선언한다. 이 변수들은 언제든지 현재 탐색 공간의 포괄적인 경계를 나타낸다. 처음에는 전체 배열을 고려한다.
2. while `left <= right`:
    - 현재 탐색 공간의 중간을 계산한다. `mid = (left + right) // 2` (버림 연산)
    - `arr[mid]`를 확인한다. 3가지 가능성이 있다:
        - `arr[mid] = x`이면, 원소를 찾았으므로 반환한다.
        - `arr[mid] > x`이면, 탐색 공간을 절반으로 줄이기 위해 `right = mid - 1`을 한다.
        - `arr[mid] < x`이면, 탐색 공간을 절반으로 줄이기 위해 `left = mid + 1`을 한다.
        - `arr[mid] = x` 없이 이 지점에 도달하면, 탐색이 실패했다. `left` 포인터는 `arr`이 정렬된 상태를 유지하기 위해 `x`가 삽입되어야 할 인덱스에 있을 것이다.

탐색 공간이 매 반복에서 절반으로 줄어들기 때문에, 이진 탐색의 최악의 경우 시간 복잡도는 $$O(\log{}n)$$이다. 이것은 로그 시간이 선형 시간에 비해 **매우** 빠르기 때문에 굉장히 강력한 알고리즘이다.

> 사람들은 인지하지 못한 채로 이진 탐색을 실생활에서 사용한다. 예를 들어, 사전에서 단어를 찾을 때, 대부분 중간쯤을 펼쳐서 해당 페이지에 있는 단어들의 첫 글자를 보고, 찾고자 하는 단어의 첫 글자에 따라 왼쪽이나 오른쪽 절반을 확인할 것이다.
{: .prompt-general }

---

## **구현 템플릿**

아래 구현 예시가 있다:

```cpp
int binarySearch(vector<int>& arr, int target) {
        int left = 0;
        int right = int(arr.size()) - 1;
        while (left <= right) {
            int mid = left + (right - left) / 2;
            if (arr[mid] == target) {
                // do something;
                return mid;
            }
            if (arr[mid] > target) {
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        
        // target이 배열에는 없지만, 'left'는 삽입 지점에 위치해 있다
        return left;
    }
```

Java와 C++ 구현에서, `(left + right) / 2`를 수행하는 대신에 `left + (right - left) / 2`를 사용하는 이유는 오버플로우를 방지하기 위해서다. 이 두 방정식은 동일하지만, 두 번째 방식은 `right`보다 큰 값이 저장되지 않도록 보장한다. 파이썬과 자바스크립트에서는 숫자가 오버플로우하지 않는다(또는 제한이 매우 크다), 그래서 `left + right`가 커도 괜찮다.

--

## **중복 요소**

입력에 중복된 요소가 있다면, 이진 탐색 템플릿을 수정해서 주어진 요소의 첫 번째 또는 마지막 위치를 찾을 수 있다. 만약 `target`이 여러 번 나타난다면, 다음 템플릿은 가장 왼쪽 인덱스를 찾는다:

```cpp
int binarySearch(vector<int>& arr, int target) {
    int left = 0;
    int right = arr.size();
    while (left < right) {
        int mid = left + (right - left) / 2;
        if (arr[mid] >= target) {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    
    return left;
}
```

다음 템플릿은 가장 오른쪽 **삽입 지점**(가장 오른쪽 요소의 인덱스에 1을 더한 값)을 찾는다:

```cpp
int binarySearch(vector<int>& arr, int target) {
    int left = 0;
    int right = arr.size();
    while (left < right) {
        int mid = left + (right - left) / 2;
        if (arr[mid] > target) {
            right = mid;
        } else {
            left = mid + 1;
        }
    }
    
    return left;
}
```

이제 이진 탐색을 어떻게 사용할 수 있는지 보여주는 예제 문제로 들어가보자.

문제가 정렬된 것을 제공할 때마다 이진 탐색에 대해 생각해야 한다. $$O(\log{}n)$$은 굉장히 빠르기 때문에 이진 탐색은 대개 큰 최적화가 된다.

---

출처: [Leetcode](https://leetcode.com/explore/interview/card/leetcodes-interview-crash-course-data-structures-and-algorithms/710/binary-search/4696/)

<!--

{: .prompt-general }

-->