---
title: "[DP] 다차원 문제"
categories: [Algorithm 연구소]
tags: [Algorithm]
date: 2023-10-07 01:30
math: true
img_path: /assets/img/algorithm/
---

---

![Title](algorithm_title.png)

---

## **개요**

이전 글에서는 상태 변수가 하나인 문제만 살펴봤다. 이 글에서는 여러 상태 변수를 사용하는 문제, 즉 다차원 DP 알고리즘을 살펴볼 것이다.

---

## **예제 1: 가장 긴 공통 부분 수열**

> [문제 링크](https://leetcode.com/problems/longest-common-subsequence/)
>
> 두 문자열 `text1`과 `text2`가 주어졌을 때, 가장 긴 공통 부분 수열의 길이를 반환하라.
>
> 예를 들어, `text1 = "abcde"` 및 `text2 = "ace"`가 주어지면, `3`을 반환한다. 두 문자열은 `"ace"`를 부분 수열로 공유한다.
{: .prompt-general }

우리는 이 문제를 DP로 해결해야 한다는 것을 어떻게 알 수 있을까? 첫째, 가장 긴 것을 요구한다. 둘째, 문자를 사용할지 말지 결정하는 것이 우리가 취할 수 있는 미래의 문자에 영향을 미친다.

문제가 가장 긴 공통 부분 수열(LCS)의 길이를 요구하기 때문에, 우리의 함수 `dp`는 LCS의 길이를 반환하도록 하자. 이번에는 두 개의 인덱스 변수가 필요하며, 각 문자열에 하나씩 - `text1`에 대해서는 `i`, `text2`에 대해서는 `j`를 사용한다. `dp(i, j)`가 `text1`의 $$i$$번째 문자와 `text2`의 $$j$$번째 문자에서 시작할 때 가장 긴 공통 부분 수열의 길이를 반환하도록 할 것이다.

각 쌍 `(i, j)`에서 두 가지 가능성이 있다:

1. `text1[i] = text2[j]`. 문자가 일치하는 경우를 찾았고 이를 사용하여 길이를 늘려야 한다. 문자를 일치시킨 후, 두 문자열의 다음 문자로 이동해야 한다. `dp(i, j) = 1 + dp(i + 1, j + 1)`. 일치하는 것을 사용하지 않는 것은 의미가 없다. 왜냐하면 어떤 주어진 단계에서 길이를 `1` 이상 늘릴 수 없기 때문에, 기회가 있을 때마다 항상 활용해야 한다.

2. `text1[i] != text2[j]`. 이제 결정을 내려야 한다. `text1`의 다음 문자로 이동할 수도 있고 `text2`의 다음 문자로 이동할 수도 있다. 두 경우 모두 시도해보는 것이 좋다 - 그래서 이 경우, `dp(i, j) = max(dp(i + 1, j), dp(i, j + 1))`.

이 두 경우는 우리의 재귀 관계를 형성한다. 종료 조건은 어떻게 될까? `i = text1.length` 또는 `j = text2.length`인 경우, 문자열 중 하나가 소진되었고 남아있는 문자가 없으므로 공통 문자가 있을 수 없다. `0`을 반환한다.

```cpp
class Solution {
public:
    string text1;
    string text2;
    
    int longestCommonSubsequence(string text1, string text2) {
        this->text1 = text1;
        this->text2 = text2;
        vector<vector<int>> memo(text1.size(), vector<int>(text2.size(), -1));
        return dp(0, 0, memo);
    }
    
    int dp(int i, int j, vector<vector<int>>& memo) {
        if (i == text1.size() || j == text2.size()) {
            return 0;
        }

        if (memo[i][j] != -1) {
            return memo[i][j];
        }
        
        int ans = 0;
        if (text1[i] == text2[j]) {
            ans = 1 + dp(i + 1, j + 1, memo);
        } else {
            ans = max(dp(i + 1, j, memo), dp(i, j + 1, memo));
        }

        memo[i][j] = ans;
        return ans;
    }
};
```

상향식, 반복은 종료 조건부터 시작해야 한다는 것을 잊지 말자:

```cpp
class Solution {
public:
    int longestCommonSubsequence(string text1, string text2) {
        int n = text1.size();
        int m = text2.size();
        vector<vector<int>> dp(n + 1, vector(m + 1, 0));
        
        for (int i = n - 1; i >= 0; i--) {
            for (int j = m - 1; j >= 0; j--) {
                if (text1[i] == text2[j]) {
                    dp[i][j] = 1 + dp[i + 1][j + 1];
                } else {
                    dp[i][j] = max(dp[i + 1][j], dp[i][j + 1]);
                }
            }
        }
        
        return dp[0][0];
    }
};
```

각 상태에서 수행되는 작업이 $$O(1)$$이므로, 이 알고리즘은 시간 및 공간 복잡도가 $$O(n \cdot m)$$이다. 여기서 `n = text1.length`이고 `m = text2.length`이다.

---

## **예제 2: 주식을 사고 팔기 가장 좋은 시간 IV**

> [문제 링크](https://leetcode.com/problems/best-time-to-buy-and-sell-stock-iv/)
>
> 정수 배열 `prices`가 주어지는데, `prices[i]`는 $$i$$번째 날의 주어진 주식의 가격이다. 그리고 정수 `k`도 있다. 주식을 사고 팔 수 있지만, 주어진 시간에 한 단위의 주식만 보유할 수 있다. 최대 `k`번의 거래로 얻을 수 있는 최대 이익을 찾아보자.
{: .prompt-general }

이 문제에서는 많은 결정을 내려야 한다. 오늘 사야 할까? 만약 산다면, 내일이나 팔 때까지 다시 사지 못한다. 팔아야 할까? 만약 판다면, 다시 사기 전까지는 팔 수 없다. 또한, 사거나 팔 때마다 제한된 거래 횟수 중 하나를 사용한다. 이 모든 결정을 내리려면 DP가 명확한 선택이다.

또한, 우리의 `dp` 함수는 답을 반환해야 한다 - 그래서 달성할 수 있는 최대 이익을 반환하게 하자. 각 상태에서 어떤 정보가 필요한가? 먼저, 당연히 몇 번째 날인지 알아야 한다. 이를 추적하기 위해 우리는 평소처럼 인덱스 변수 `i`를 사용할 수 있다. 다음으로, 현재 주식을 보유하고 있는지 알아야 한다. 주식을 보유하고 있는지를 나타내는 부울 값인 `holding`을 사용하자. 마지막으로, 문제에는 명시적인 수치 제약이 있다 - 우리가 허용되는 거래 횟수다. 남아있는 거래 횟수를 나타내는 정수 `remain`도 사용한다.

우리는 `dp(i, holding, remain)`이 현재 `i`일에 있고, 남아있는 거래가 `remain`이며, `holding`이 현재 주식을 보유하고 있는지를 나타내는 경우 달성할 수 있는 최대 이익을 반환하도록 할 것이다. 답은 `dp(0, false, k)`가 되어야 하는데, 이는 `0`일에 시작하여 주식을 보유하지 않고, `k`번의 거래가 남아 있는 상태에서 달성할 수 있는 최대 이익을 반환한다.

각 상태에서 우리가 할 수 있는 결정은 무엇인가? 주식을 보유하고 있지 않다면, 오늘 사거나 건너뛸 수 있다. 사면, 이익은 `-prices[i] + dp(i + 1, true, remain)`이 된다. 우리는 `prices[i]`를 지불하여 주식을 산다. 그런 다음 다음 날로 이동(`i + 1`), 이제 주식을 보유하고 있으며(`true`), 아직 거래를 완료하지 않았기 때문에 `remain`은 동일하게 유지된다.

주식을 보유하고 있다면, 오늘 팔거나 건너뛸 수 있다. 만약 판다면, 이익은 `prices[i] + dp(i + 1, false, remain - 1)`이다. `prices[i]`의 돈을 번다. 그런 다음 다음 날로 이동(`i + 1`), 우리는 더 이상 주식을 보유하고 있지 않으며(`false`), 우리의 거래 중 하나를 사용했다(`remain - 1`).

두 경우 모두, 건너뛰기로 결정하면 이익은 `dp(i + 1, holding, remain)`이다. 다른 것은 변하지 않고 다음 날로 그냥 이동한다. 따라서, 우리의 재귀 관계는 다음과 같다:

`dp(i, holding, remain) = max(skip, buy, sell)` 이며,

`skip = dp(i + 1, holding, remain)`,

`buy = -prices[i] + dp(i + 1, true, remain)` 이지만 `holding = false`인 경우에만 고려된다,

`sell = prices[i] + dp(i + 1, false, remain - 1)` 이지만 `holding = true` 경우에만 고려된다.

종료 조건은 무엇인가? 입력의 끝에 도달하면(`i = prices.length`), 더 이상 거래를 할 수 없으므로 `0`을 반환한다. 거래를 다 사용하면(`k = 0`), 더 이상 거래를 할 수 없으므로 `0`을 반환한다.

> `holding`에 대해서는, `false`를 나타내는 `0`과 `true`를 나타내는 `1`을 사용할 수 있다.
{: .prompt-general }

```cpp
class Solution {
public:
    vector<int> prices;
    vector<vector<vector<int>>> memo;
    
    int maxProfit(int k, vector<int>& prices) {
        this->prices = prices;
        memo = vector(prices.size(), vector(2, vector(k + 1, -1)));
        
        return dp(0, 0, k);
    }
    
    int dp(int i, int holding, int remain) {
        if (i == prices.size() || remain == 0) {
            return 0;
        }
        
        if (memo[i][holding][remain] != -1) {
            return memo[i][holding][remain];
        }
        
        int ans = dp(i + 1, holding, remain);
        if (holding == 1) {
            ans = max(ans, prices[i] + dp(i + 1, 0, remain - 1));
        } else {
            ans = max(ans, -prices[i] + dp(i + 1, 1, remain));
        }
        
        memo[i][holding][remain] = ans;
        return ans;
    }
};
```

여기 싱향식 구현이 있다. for 루프를 설정하는 것이 까다로울 수 있다는 것을 알 수 있다. 기억해야 할 중요한 것은 기본 사례에서 반복을 시작해야 한다는 것이므로, `n`에서 `i`를, `1`에서 `remain`을 반복하여 시작한다. 여기서 `n`은 `prices.length`다.

```cpp
class Solution {
public:
    int maxProfit(int k, vector<int>& prices) {
        int n = prices.size();
        vector<vector<vector<int>>> dp(n + 1, vector(2, vector(k + 1, 0)));
        for (int i = n - 1; i >= 0; i--) {
            for (int remain = 1; remain <= k; remain++) {
                for (int holding = 0; holding < 2; holding++) {
                    int ans = dp[i + 1][holding][remain];
                    if (holding == 1) {
                        ans = max(ans, prices[i] + dp[i + 1][0][remain - 1]);
                    } else {
                        ans = max(ans, -prices[i] + dp[i + 1][1][remain]);
                    }
                    
                    dp[i][holding][remain] = ans;
                }
            }
        }
        
        return dp[0][0][k];
    }
};
```

각 상태에서 수행되는 작업은 $$O(1)$$이므로, 시간과 공간 복잡도는 상태 수와 같다. 각 상태 변수의 범위의 곱은 상태 수다 - 이것은 우리에게 $$O(n \cdot k)$$의 시간과 공간 복잡도를 준다. `holding`은 일정하며, `n`은 `i`에서 왔고, `k`는 `remain`에서 왔다.

---

## **예제 3: 더미에서 K개의 동전으로 최대 가치 얻기**

> [문제 링크](https://leetcode.com/problems/maximum-value-of-k-coins-from-piles/)
>
> 테이블 위에는 동전 더미가 `n`개 있다. 각 더미는 여러 종류의 동전으로 구성되어 있으며, 양수 개의 동전이 포함되어 있다. 한 번의 움직임으로, 어떤 더미의 맨 위에 있는 동전을 선택하고, 그것을 제거하며, 지갑에 추가할 수 있다. `piles`라는 리스트가 주어지는데, `piles[i]`는 위에서 아래로 $$i$$번째 더미의 구성을 나타내는 정수 리스트다. 양수 정수 `k`도 있다. 정확히 `k`개의 동전을 최적으로 선택할 경우 지갑에 넣을 수 있는 동전의 최대 총 가치를 반환해야 한다.
{: .prompt-general }

이제 규칙은 알고 있을 거다 - 가질 수 있는 가장 많은 돈을 반환하는 함수 `dp`를 사용할 것이다. 우리는 현재 어떤 더미에 있는지를 나타낼 수 있는 평소처럼 인덱스 변수 `i`가 있고, 문제에는 `k`라는 명시적인 숫자 제약도 있으니, 더 가져갈 수 있는 동전의 수를 나타내는 다른 상태 변수 `remain`을 사용할 것이다. 그래서 우리는 `dp(i, remain)`을 가지고 있는데, 이것은 남아있는 이동 횟수가 `remain`일 때 i번째 더미에서 시작하여 취할 수 있는 동전의 최대 가치를 반환한다.

$$i$$번째 더미에서, 우리는 더미를 건너뛰거나 일부 동전을 가져갈 수 있다. 건너뛰면 점수는 `dp(i + 1, remain)`이다. 건너뛰지 않으면, 몇 개의 동전을 가져갈지 선택할 수 있다. $$j$$번째 동전까지 가져가면, 점수는 `piles[i][:j]`의 합계에 `dp(i + 1, remain - j - 1)`을 더한 것과 같다. `remain` 동전보다 많이 가져가지 않도록 해야 한다.

각 상태에서 모든 가능성을 시도해야 한다. 주어진 상태에서, 현재 더미에서 가져간 동전의 가치를 추적하기 위해 정수 변수 `curr`를 사용할 수 있고, 더미를 순회하면서 가능한 최대 점수를 찾을 수 있다. 재귀 관계는 다음과 같다:

`dp(i, remain) = max(skip, take)`, 여기서

`skip = dp(i + 1, remain)`, 그리고

`take = max( sum(piles[i][:j]) + dp(i + 1, remain - j - 1) for j from 0 to min(remain, piles[i].length) )`

이것은 꽤 무섭게 보이지만, 각 용어를 따로 보면 더 단순해진다. `sum(piles[i][:j])`은 현재 더미에서 가져간 동전의 가치이고, `j + 1`은 우리가 가져간 동전의 수이므로, 우리에게 `remain - j - 1`의 이동이 남아 있고, `min(remain, piles[i].length)`은 우리가 허용된 것보다 많은 동전을 가져가지 않도록 하는 것이다.

종료 조건은 무엇인가? 입력의 끝에 도달하면(더 이상 더미가 없음) 또는 `remain = 0`(더 이상 동전을 가져갈 수 없음)이면, 달성할 수 있는 최대 점수는 `0`이다.

```cpp
class Solution {
public:
    vector<vector<int>> piles;
    vector<vector<int>> memo;
    
    int maxValueOfCoins(vector<vector<int>>& piles, int k) {
        this->piles = piles;
        memo = vector(piles.size(), vector(k + 1, -1));
        return dp(0, k);
    }
    
    int dp(int i, int remain) {
        if (i == piles.size() || remain == 0) {
            return 0;
        }
        
        if (memo[i][remain] != -1) {
            return memo[i][remain];
        }
        
        int ans = dp(i + 1, remain); // skip this pile
        int curr = 0;
        int pileSize = piles[i].size();
        
        for (int j = 0; j < min(remain, pileSize); j++) {
            curr += piles[i][j];
            ans = max(ans, curr + dp(i + 1, remain - j - 1));
        }
        
        memo[i][remain] = ans;
        return ans;
    }
};
```

상향식 해결 방법에서, 기본 사례인 `i = n` 및 `remain = 0`에서 반복을 시작하는 것을 기억하자:

```cpp
class Solution {
public:
    int maxValueOfCoins(vector<vector<int>>& piles, int k) {
        int n = piles.size();
        vector<vector<int>> dp(n + 1, vector(k + 1, 0));
        for (int i = n - 1; i >= 0; i--) {
            for (int remain = 1; remain <= k; remain++) {
                dp[i][remain] = dp[i + 1][remain]; // skip this pile
                int curr = 0;
                int pileSize = piles[i].size();
                for (int j = 0; j < min(remain, pileSize); j++) {
                    curr += piles[i][j];
                    dp[i][remain] = max(dp[i][remain], curr + dp[i + 1][remain - j - 1]);
                }
            }
        }
        
        return dp[0][k];
    }
};
```

각 더미의 평균 동전 수가 x라고 하자. $$O(n \cdot k)$$ 상태가 있고, 각 상태에서 `x`번 반복하는 for 루프가 있다. 이것은 $$O(n \cdot k \cdot x)$$의 시간 복잡도와 $$O(n \cdot k)$$의 공간 복잡도를 제공한다.

---

동적 프로그래밍 문제는 어려울 수 있다 - 하지만 프레임워크를 사용하고 각 구성 요소에 대해 신중하게 생각함으로써, 알고리즘은 체계적인 방법으로 구성될 수 있다.

---

출처: [Leetcode](https://leetcode.com/explore/interview/card/leetcodes-interview-crash-course-data-structures-and-algorithms/712/dynamic-programming/4542/)

<!--

{: .prompt-general }

-->