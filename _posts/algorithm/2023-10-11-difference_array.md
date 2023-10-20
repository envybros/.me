---
title: "차이 배열"
categories: [Algorithm 연구소]
tags: [Algorithm]
date: 2023-10-11 01:30
math: true
img_path: /assets/img/algorithm/
---

---

![Title](algorithm_title.png)

---

## **개요**

차이 배열은 매우 흔하게 사용되는 것은 아니지만, 면접에서 즉석에서 이 아이디어를 도출하기는 어려우므로 알아두면 좋다.

차이 배열은 이벤트 간격과 관련된 문제를 해결할 때 사용된다. 모든 이벤트가 숫자 선을 따라 발생하는 것으로 생각할 수 있다. 이 숫자 선은 시간 또는 위치를 나타낼 수 있다. 이벤트는 이 숫자 선에서 시작점과 끝점을 가지고 발생한다.

입력은 각 내부 배열이 `[left, right, value]` 형식인 2D 배열을 제공하거나, 일부 동등한 형태를 제공할 것이다. 문제의 배경은 대개 "`left`과 `right` 사이에 어떤 `value`이 있다"와 같은 식일 것이다. 몇 가지 예를 살펴보자.

---

출처: [Leetcode](https://leetcode.com/explore/interview/card/leetcodes-interview-crash-course-data-structures-and-algorithms/714/bonus/4688/)

<!--

{: .prompt-general }

-->