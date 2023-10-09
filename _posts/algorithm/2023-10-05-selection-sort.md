---
title: "선택 정렬 (Selection Sort)"
categories: [Algorithm 연구소]
tags: [Algorithm]
date: 2023-10-05 00:20
math: true
img_path: /assets/img/algorithm/
---

---

![Title](algorithm_title.png)

---
## 개요

> ![Selection](SelectionSort_Avg_case.gif)   
> 분류: **정렬 알고리즘**  
> 자료 구조: **배열**  
> 최악 시간복잡도: 비교: $$O(n^2)$$, 교환: $$O(n)$$  
> 최선 시간복잡도: 비교: $$O(n^2)$$, 교환: $$O(n)$$  
> 평균 시간복잡도: 비교: $$O(n^2)$$, 교환: $$O(n)$$  
> 공간복잡도:예비: $$O(1)$$  
{: .prompt-general }

**선택 정렬**은 제자리 정렬 알고리즘 중 하나이다. 비교가 상수 시간이 걸린다는 가정 하에 선택 정렬은 $$O(n^2)$$만큼의 시간이 걸린다. 알고리즘이 매우 단순하고, 메모리가 제한적인 경우 사용시 성능상의 이점이 있다는 특징이 있다.

---

## 알고리즘

> ![Selection2](selectionSort.gif)
{: .prompt-img }

1. 배열의 가장 작은 요소를 찾는다.
2. 가장 작은 요소를 첫 번째 요소와 교환한다.
3. 이제 두 번째로 작은 요소를 찾아 두 번째 위치의 요소와 교환한다.
4. 전체 배열이 정렬될 때까지 반복한다.

---

## 구현

선택 정렬에 대한 cpp 구현은 다음과 같다.

```cpp
void swap(int &a, int &b) {
    int tmp = a;
    a = b;
    b = tmp;
}

void selectionSort(int arr[], int n) {
    // n-1: 마지막 요소는 검사할 필요가 없다.
    for (int i = 0; i < n-1; i++) {
        int minIdx = i; // 현재 위치를 최소 값의 인덱스로 초기화한다.

        // 다음 요소부터 배열의 끝까지 최소 값을 찾는다.
        for (int j = i+1; j < n; j++) {
            if (arr[j] > arr[i]) {
                swap(arr[j], arr[j+1]);
            }
        }
    }
}

```

---

## 시간 복잡도

각 비교에 소요되는 시간을 상수 $$C$$라고 생각해보자. 

그렇다면 버블 정렬에 소요되는 총 시간은 $$C*(N-1 + N-2 + ... + 2 + 1)$$이다.

즉, 시간 복잡도는 $$O(n^2)$$ 이다.

---

## 공간 복잡도

버블 정렬에서는 추가적인 공간을 사용하지 않기 때문에 공간 복잡도는 $$O(1)$$이다.

---

## 장점

- 데이터 세트의 크기가 작은 경우 효율적이다.
- 구현이 매우 간단하다.
- 추가 메모리를 요소하지 않으므로, 메모리 효율적이다.
- 안정적인 정렬을 제공한다. (실패률이 거의 없다.)

---

## 단점

- 시간 복잡도가 $$O(n^2)$$로 큰 편이다.
- 데이터 세트의 크기가 큰 경우, 시간이 기하급수적으로 증가하기 때문에 효율성이 떨어진다.

---

참고 자료:

- [codepumpkin](https://codepumpkin.com/bubble-sort/)
- [wiki](https://ko.wikipedia.org/wiki/%EB%B2%84%EB%B8%94_%EC%A0%95%EB%A0%AC)
