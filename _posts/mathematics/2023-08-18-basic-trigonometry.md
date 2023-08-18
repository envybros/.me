---
title: "Basic Trigonometry"
categories: [Unity 연구소]
tags: [Unity, Math]
date: 2023-08-18 00:20
math: true
img_path: /assets/img/unity/math
---

---

## **기본 삼각법 (삼각 함수)**

본격적으로 들어가기 전에 삼각형과 삼각형의 속성과 기능은 게임 수학에 있어 필수 요소이다.

---

## **각도 (Angles)**

각은 두 선이 교차하여 형성된 도형이다. 각도에는 세 가지 기본 타입이 있는데, 이는 90º를 기준으로 구분지을 수 있다.

![Angle](angle.png)
{: .prompt-img }

- **예각(Acute)**: 90º 미만의 각도
- **직각(perpendicular)**: 한 선이 다른 선과 90º로 만나는 것
- **둔각(Obtuse)**: 90º보다 큰 각도

---

## **삼각형 (Triangles)**

삼각형에는 부등변, 직각, 그리고 이등변이 있다.

![Triangles](triangle.png)
{: .prompt-img }

- **부등변 삼각형(Scalene)**: 길이가 같은 변이 없는 삼각형
- **이등변 삼각형(Isosceles)**: 길이가 같은 두 변이 있는 삼각형
- **직각 삼각형(Right angle)**: (게임 수학에서 가장 강력한 힘을 제공하는) 내부 각도가 90º인 삼각형.(항상 모서리에 90º인 정사각형 상자가 그려져 있다)

*삼각형의 모든 내각의 합은 무조건 180º이다.*

---

## **직각 삼각형의 성질**

![Right Angle](right_angle.png)
{: .prompt-img }

직각 삼각형은 가장 긴 변(직각의 반대쪽 변)의 **빗변(Hypotenuse)**이라 한다. 또한 세 변과 각도의 관계에 따라, 다른 두 변을 **밑변(Adjacent)**, 그리고 **높이(Opposite)**라고 한다. 예를 들어 각도 β의 경우 b는 높이(Opposite)라고 하고, a는 밑변(Adjacent)라 할 수 있다. 물론 다른 각도를 기준으로 보면 a와 b의 역할이 뒤바뀐다.

직각 삼각형의 장점은 a, b, 그리고 빗변(Hypotenuse)의 길이를 알면 모든 내각의 길이를 계산할 수 있다는 것이다.

공식은 다음과 같다.

$$sin(θ) = \frac{높이(Opposite)}{빗변(Hypotenuse)}$$

$$cos(θ) = \frac{밑변(Adjacent)}{빗변(Hypotenuse)}$$

$$tan(θ) = \frac{높이(Opposite)}{밑변(Adjacent)}$$

예를 들어, *a = 3*이고 *h(빗변)이 6*이라고 가정해보자. **β**의 밑변과 빗변이 주어졌으므로, 우리는 β의 각도를 계산할 수 있다. 따라서

$$cos(β) = \frac{3}{6}$$

이는 cose(β) = 0.5와 같다. 이제 이 값을 계산하기 위해서는 역 코사인(inverse cosine)을 사용해야 한다:

$$β = cos^-1(0.5)$$

만약 계산기가 없다면, Google에 *type = acos(0.5)*를 입력해보자. 해당 답을 찾을 수 있을 것이다. acos() 함수를 통해 코사인 함수의 역함수를 구할 수 있다. 사인 함수에 대해서는 asin() 이 있다.

$$acos(0.5) = 1.04719755 rad$$

(참고로 acos() 함수는 수학시간에 배웠던 아크코사인(arccos)와 같다.)

0.5의 아크코사인은 1.042 라디안(radians)이다. 이를 도(度)로 변환하기 위해선 **180/PI**를 곱해야 한다. 바로 변환해보면 β의 각도는 60º가 된다.

이제 삼각형 자체를 확인해보자. 아마 각도기가 있다면, 직접 측정해볼 수도 있을 것이다.

---

자료: [Penny de Byl](http://www.holistic3d.com/) 선생님의 "Mathematics for Computer Games Development using Unity"
