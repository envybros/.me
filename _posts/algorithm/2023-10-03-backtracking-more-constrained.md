---
title: "[백트래킹] 더 제한된 상황"
categories: [Algorithm 연구소]
tags: [Algorithm]
date: 2023-10-03 02:30
math: true
img_path: /assets/img/algorithm/
---

---

![Title](algorithm_title.png)

---

## **개요**

생성 문제는 비교적 간단하며 보통 같은 형식을 따른다. 다른 백트래킹 문제들도 살펴보자.

---

## **예제 1: 조합의 합**

> [문제 링크](https://leetcode.com/problems/combination-sum/)
>
> 서로 다른 양의 정수로 이루어진 배열 `candidates`와 목표 정수 `target`이 주어지면, 선택한 숫자들의 합이 `target`이 되는 `candidates`의 모든 고유한 조합 목록을 반환하라. 같은 숫자를 `candidates`에서 무한정 선택할 수 있다. 적어도 하나의 선택한 숫자의 빈도가 다르면 두 조합은 고유하다.
{: .prompt-general }

문제가 조합을 생성하도록 요구하기 때문에, 이전 글에서 사용한 일부 논리를 재사용할 수 있다. 예를 들어 `[2, 2, 3]`과 `[2, 3, 2]`와 같은 중복을 피하기 위해서, 우리는 반복을 시작하는 위치를 나타내는 정수 변수 `start`를 전달해야 한다(이전 글의 `i`와 같은 목적). 이전 문제들에서, 우리가 $$i$$번째 요소를 추가하고 있다면, 동일한 요소를 한 번 이상 사용할 수 없기 때문에 `i + 1`을 전달했다. 이 문제에서는 동일한 요소를 여러 번 사용할 수 있으므로, `i`를 전달해야 한다.

`target`에 합산되는 조합을 찾아야 한다. 현재 합계를 추적하기 위해, 우리는 정수 변수 `curr`를 전달할 수 있다. `curr`에 더하여 `target`을 초과하게 만들면, 그 경로를 더 이상 탐색해서는 안 된다는 것을 알 수 있다. `curr == target`이라면, 우리의 답변에 추가할 수 있다는 것을 알 수 있다.

### [예제 1] 상세 설명

이 문제에서는 `path`라는 인자를 사용하여 현재 경로에 있는 숫자들을 나타낸다(이전 글에서 `curr`가 했던 일). 또한 `start`라는 인자를 가지고 입력에서 반복을 시작할 위치를 나타낼 것이다(이전 글에서 `i`가 했던 일). `curr`는 `path`에 있는 모든 숫자의 합계를 나타낸다. 이것은 필수적이지 않다. 왜냐하면 `path`에서 그것을 계산할 수 있기 때문이지만, 모든 노드에서 처음부터 계산할 필요가 없어 알고리즘을 더 효율적으로 만들기 때문이다.

원소를 `path`에 추가하고 자식 노드로 이동할 때, `curr`에 그 값을 추가해야 한다. 반대로 트리를 다시 올라가서 `path`에서 원소를 제거할 때, `curr`에서 그 값을 빼야 한다. 만약 원소를 추가하면 `curr`가 `target`을 초과하게 되면, 그 원소를 추가해서는 안 된다. 입력에는 음수가 없으므로, 만약 `target`을 초과하면 절대 도달할 수 없다.

기본적인 상황은 `curr = target`일 때이다. 이 경우가 리프 노드가 될 수 있지만, 모든 리프 노드가 그런 상황은 아니다. `curr`가 `target`보다 작지만, 어떤 숫자도 추가하면 `curr`가 `target`을 초과하게 만들면, 그 노드도 리프 노드가 된다. `curr = target`인 경우에만 `path`를 정답에 추가해야 한다.

```cpp
class Solution {
public:
    vector<int> nums;
    int target;
    vector<vector<int>> ans;
    
    vector<vector<int>> combinationSum(vector<int>& candidates, int target) {
        nums = candidates;
        this->target = target;
        vector<int> path = {};
        backtrack(path, 0, 0);
        return ans;
    }
    
    void backtrack(vector<int>& path, int start, int curr) {
        if (curr == target) {
            ans.push_back(path);
            return;
        }
        
        for (int i = start; i < nums.size(); i++) {
            int num = nums[i];
            if (curr + num <= target) {
                path.push_back(num);
                backtrack(path, i, curr + num);
                path.pop_back();
            }
            
        }
    }
};
```

이 알고리즘의 시간 복잡도는 대략 $$O(n^{\frac{T}{M}})$$로, 이는 `n = candidates.length`, `T = target`, 그리고 `M = min(candidates)`이기 때문이다. 일반적으로 재귀는 트리로 생각할 수 있다는 것을 기억해라. 이 문제에서 트리의 최대 깊이는 $$\frac{T}{M}$$다 - 가장 작은 숫자를 반복 사용하여 `target`을 초과할 때까지다. 트리의 각 노드는 최대 `n`개의 자식을 가질 수 있어, 이것이 $$O(n^{\frac{T}{M}})$$를 제공한다.

출력을 추가 공간으로 계산하지 않는다면, 이 문제에서 사용되는 공간은 `path`와 재귀 호출 스택 두 가지 모두 $$O(\frac{T}{M})$$이다.

---

## **예제 2: N-퀸 II**

> [문제 링크](https://leetcode.com/problems/n-queens-ii/)
>
> 이 문제에서는 `n`개의 퀸을 `n x n` 체스판에 배치하는 것이다. 단, 어떠한 두 퀸도 서로 공격할 수 없어야 한다. 정수 `n`이 주어지면, N-Queens 퍼즐의 고유한 해결책 수를 반환한다.
>
> 체스를 하지 않는 사람들을 위한 설명: 퀸은 자신이 위치한 행, 열, 대각선을 따라 공격할 수 있다.
{: .prompt-general }

n-queens 퍼즐은 클래식한 백트래킹 예제로, 대학이나 대학 강의에서 백트래킹을 가르치기 위해 자주 사용되는 문제다. 이 문제에서는 `n`개의 퀸을 배치해야 하는데, 각 행, 열, 대각선, 그리고 반대각선에는 퀸이 하나씩만 있어야 한다.

행부터 시작해보자. 이건 쉽다. 각 `backtrack` 호출마다 한 행씩 고려할 수 있다. `row`를 인수로 전달하고, `row`가 `n`과 같아지면 유효한 해결책을 찾았다는 것을 알 수 있다.

열에 대해서는: 열은 `[0, n)`에서 번호가 매겨지고, 각 열에는 퀸이 하나만 있어야 한다. 이미 차지된 열을 저장하는 집합 `cols`를 사용할 수 있다. 퀸을 추가할 때는 해당 열을 `cols`에 추가하고, 백트래킹을 통해 퀸을 제거할 때는 해당 열을 `cols`에서 제거한다.

대각선과 반대 대각선에 대해서는 (대각선은 왼쪽 상단에서 오른쪽 하단으로, 반대 대각선은 오른쪽 상단에서 왼쪽 하단으로 진행), 굉장히 똑똑한 방법을 사용할 수 있다.

대각선 상의 모든 칸은 `row - col` 값이 같다. 대각선은 아래쪽과 오른쪽으로 이동하기 때문에 `row`와 `col`이 모두 증가하지만, 그 차이는 변하지 않는다. 열과 마찬가지로 동일한 대각선에 여러 퀸을 배치하지 않도록 집합을 사용할 수 있다.

![nqueen2](nqueen2.png)

반대 대각선에 있는 모든 칸은 `row + col` 값이 같다. 반대 대각선은 아래쪽과 왼쪽으로 이동하기 때문에 `row`는 증가하고 `col`은 감소하지만, 그 합계는 변하지 않는다. 열과 대각선과 마찬가지로 동일한 반대 대각선에 여러 퀸을 놓지 않도록 집합을 사용할 수 있다.

![nqueen3](nqueen3.png)

각 행마다, 우리는 열을 반복 처리할 수 있고, 현재 칸의 대각선과 반대 대각선의 값을 계산한 다음, 이미 세 곳 중 어느 곳도 점유되어 있지 않은지 확인할 수 있다. 구현 측면에서는 일반적인 형식을 사용할 수 있다.

### [예제 2] 상세 설명

이 장에서 지금까지 살펴본 문제들은 우리가 작업해야 할 정수 배열과 같은 매우 잘 정의된 입력을 가지고 있었다. 이 문제에서는 코드로 표현하기에 좀 더 어려운 체스판을 다루고 있다. 이 문제를 해결하는 한 가지 방법은 체스판을 글자 그대로 저장하는 백트래킹 함수에 `n * n` 행렬을 인수로 전달하는 것일 수 있다. 그러나 이것은 필요하지 않고 매우 비효율적이다. 체스판을 나타내기 위해 필요한 것보다 적은 양을 사용하려고 노력해야 한다. 이 문제에서는 퀸들이 서로 공격하는 것에만 관심이 있다. 퀸을 놓으려고 할 때 현재 상태에서 공격받지 않는 칸에 놓아야 한다. 퀸이 차지하는 행, 열, 대각선을 공격하기 때문에 각각에 대해 인수를 사용하기만 하면 된다. 위에서 언급했듯이, 대각선과 관련된 깔끔한 트릭을 사용하여 대각선이 이미 공격받고 있는지 효율적으로 찾을 수 있다. 또한 현재 `row`에 대해 정수만 사용하고, 행 당 퀸 하나만 놓으면 된다.

보드를 어떻게 나타낼지 알면, 일반적인 백트래킹 템플릿을 사용하기만 하면 된다. 각 행에 대해, 칸이 공격받지 않았다면 모든 열에 퀸을 놓아보려고 시도한다. 퀸을 "놓음"으로써 새로 놓인 퀸이 공격하는 열/대각선을 나타내기 위해 상태 변수를 업데이트한다. 백트래킹할 때, 퀸을 보드에서 "제거"함으로써 퀸을 놓을 때 만든 업데이트를 취소한다. `row = n`일 때 (퀸을 `n`개 놓았을 때) 유효한 해결책을 가지고 있다는 것을 알 수 있다.

```cpp
class Solution {
public:
    int n;
    
    int totalNQueens(int n) {
        this->n = n;
        unordered_set<int> diagonals;
        unordered_set<int> antiDiagonals;
        unordered_set<int> cols;
        return backtrack(0, diagonals, antiDiagonals, cols);
    }
    
    int backtrack(int row, unordered_set<int>& diagonals, unordered_set<int>& antiDiagonals, unordered_set<int>& cols) {
        // 종료 조건 - N개의 퀸이 배치됨
        if (row == n) {
            return 1;
        }
        
        int solutions = 0;
        for (int col = 0; col < n; col++) {
            int currDiagonal = row - col;
            int currAntiDiagonal = row + col;
            
            // 퀸을 놓을 수 없는 경우
            if (cols.find(col) != cols.end() ||
               diagonals.find(currDiagonal) != diagonals.end() ||
               antiDiagonals.find(currAntiDiagonal) != antiDiagonals.end()) {
                continue;
            }
            
            // 보드에 퀸을 "추가"
            cols.insert(col);
            diagonals.insert(currDiagonal);
            antiDiagonals.insert(currAntiDiagonal);
            
            // 업데이트된 보드 상태로 다음 행으로 이동
            solutions += backtrack(row + 1, diagonals, antiDiagonals, cols);
            
            // 위의 함수 호출을 사용하여 모든 유효한 경로를 이미 탐색했기 때문에
            // 보드에서 퀸을 "제거"
            cols.erase(col);
            diagonals.erase(currDiagonal);
            antiDiagonals.erase(currAntiDiagonal);
        }
        
        return solutions;
    }
};
```

이 알고리즘의 실제 시간 복잡도는 사실 알려져 있지 않지만, 대략 $$O(n!)$$ 정도로 추정된다. 첫 번째 호출에서 `n`개의 옵션을 고려하게 되고, 다음 호출에서는 같은 열에 하나의 칸이 배치될 것이고, 대각선/역대각선에 적어도 하나의 칸이 배치될 것이므로 `n - 2`개의 옵션을 고려하게 된다. 이 패턴은 계속되어, 그 다음 호출에서는 `n - 4`개의 칸을 고려하게 될 것이다. 공간 복잡도는 세트와 재귀 호출 스택 때문에 $$O(n)$$이다.

---

## **예제 3: 단어 검색**

> [문제 링크](https://leetcode.com/problems/word-search/)
>
> 문자의 `m x n` 그리드 보드와 문자열 `word`가 주어지면, `word`가 그리드에 존재하는지 여부를 반환한다. `word`는 인접한 셀의 문자로 구성될 수 있으며, 인접한 셀은 수평적 또는 수직적으로 이웃해야 한다. 같은 문자 셀은 한 번 이상 사용될 수 없다.
{: .prompt-general }

![word_search](word_search.png)

입력값은 익숙해 보일 것이다. 각 칸이 노드이고 인접한 칸 사이에 에지가 있는 그래프다. 이 문제가 단순한 DFS가 아닌 백트래킹인 이유는 동일한 초기 함수 호출에서 한 칸을 여러 번 방문할 수 있기 때문이다. 답을 얻기 위해 한 칸을 한 번 이상 사용할 수는 없지만, 다른 후보를 형성하기 위해 한 칸을 여러 가지 방법으로 사용할 수 있다. 예를 들어, `(0, 0)`에서 시작하여 오른쪽으로 가고 나서 아래로 가는 경로는 아래로 가고 나서 오른쪽으로 가는 경로와 다르다.

DFS에서 했던 것처럼 같은 경로에서 같은 글자를 사용하지 않도록 집합 `seen`을 사용해야 하지만, DFS와 달리 백트래킹할 때는 이 집합에서 제거해야 한다. 경로가 우리에게 답을 줄 수 있다는 것을 알 때만 한 에지를 순회해야 하므로, 현재 `word[i]`를 찾고 있다고 표시하는 인덱스 변수 `i`도 전달할 수 있다. 그런 다음 올바른 글자인 경우에만 `(nextRow, nextCol)`로 이동한다. 답이 어떤 칸에서든 시작할 수 있으므로, `word[0]`이 있는 모든 칸에서 백트래킹을 시작해야 한다.

요약하자면, 같은 경로에서 한 칸을 여러 번 사용하지 않도록 집합을 사용하여 경로를 순회하는 DFS와 유사한 알고리즘을 사용한다. 다음 글자가 있는 칸으로만 이동하면서 천천히 단어를 구성해 나간다.

### [예제 3] 상세 설명

우리가 그래프를 가지고 있기 때문에, 행렬을 순회하기 위해 DFS와 유사한 알고리즘을 수행할 수 있다. `word[0]` 값을 가진 칸을 선택하고 거기서 시작한다. 그런 다음 이웃에서 `word[1]`을 찾는다. 찾으면 그곳으로 이동하고 `word[2]`를 찾고, 단어를 완성하거나 단어를 계속 만들 수 없는 지점에 도달할 때까지 계속한다. 계속할 수 없다는 것을 깨닫게 되면, 보통 DFS에서 하는 것처럼 이전 노드로 돌아가지만, 다시 방문하는 것을 피하기 위해 사용 중인 집합에서 떠나는 칸도 제거해야 한다. 이 집합은 주어진 경로에서 한 글자를 한 번 이상 사용하는 것을 방지하지만, 경로를 포기하기 시작하면 그것을 제거해서 미래의 경로에서 사용될 수 있도록 해야 한다.

```cpp
class Solution {
public:
    int m;
    int n;
    string target;
    vector<vector<int>> directions = {{0, 1}, {1, 0}, {0, -1}, {-1, 0}};
    vector<vector<bool>> seen;
    
    bool exist(vector<vector<char>>& board, string word) {
        m = board.size();
        n = board[0].size();
        target = word;
        seen = vector(m, vector<bool>(n, false));
        
        for (int row = 0; row < m; row++) {
            for (int col = 0; col < n; col++) {
                if (board[row][col] == word[0]) {
                    seen[row][col] = true;
                    if (backtrack(row, col, 1, board)) {
                        return true;
                    }
                    seen[row][col] = false;
                }
            }
        }
        
        return false;
    }
    
    bool backtrack(int row, int col, int i, vector<vector<char>>& board) {
        if (i == target.size()) {
            return true;
        }
        
        for (vector<int>& direction: directions) {
            int nextRow = row + direction[0], nextCol = col + direction[1];
            if (valid(nextRow, nextCol) && !seen[nextRow][nextCol]) {
                if (board[nextRow][nextCol] == target[i]) {
                    seen[nextRow][nextCol] = true;
                    if (backtrack(nextRow, nextCol, i + 1, board)) {
                        return true;
                    }
                    seen[nextRow][nextCol] = false;
                }
            }
        }
        
        return false;
    }
    
    bool valid(int row, int col) {
        return 0 <= row && row < m && 0 <= col && col < n;
    }
};
```

이 알고리즘의 시간 복잡도는 $$O(n \cdot m \cdot 3^L)$$이다. 여기서 `L`은 `word.length`이다. 해결책 공간 트리를 생각해보면, 각 노드는 최대 3개의 자식을 가질 수 있다(4개가 아니다, 왜냐하면 우리는 이전에 왔던 칸을 고려하지 않기 때문에, 각 노드는 최대 3개의 자식을 가진다, 첫 번째 노드 제외). 최대 깊이는 L이므로 이는 $$3^L$$개의 노드를 제공한다. 또한, 우리는 시작할 수 있는 $$n \cdot m$$개의 칸이 있다. 이 시간 복잡도는 최악의 경우에 발생하는데, 그리드에 한 글자만 있고, 단어도 그 한 글자만 있으며, 마지막에 다른 글자가 추가된 경우다. 예를 들어, `board = [["A", "A", "A"],["A", "A", "A"],["A", "A", "A"]]`, `word = "AAAAAAAAZ"`. 공간 복잡도는 재귀 호출 스택과 `seen`이 세트인 경우 $$O(L)$$이다. bool 배열을 사용하는 경우, 공간 복잡도는 대신 $$O(n \cdot m)$$이다.

우리가 해결한 예제들에서 볼 수 있듯이, 백트래킹 문제들은 모두 매우 비슷한 형식을 따른다. 문제들 사이의 주요 차이점은 각 "노드"의 상태를 어떻게 나타내느냐에 있다.

---

## **보너스 문제**

- [47. Permutations II](https://leetcode.com/problems/permutations-ii/)
- [40. Combination Sum II](https://leetcode.com/problems/combination-sum-ii/)
- [2305. Fair Distribution of Cookies](https://leetcode.com/problems/fair-distribution-of-cookies/)
- [1415. The k-th Lexicographical String of All Happy Strings of Length n](https://leetcode.com/problems/the-k-th-lexicographical-string-of-all-happy-strings-of-length-n/)
- [491. Non-decreasing Subsequences](https://leetcode.com/problems/non-decreasing-subsequences/)
- [93. Restore IP Addresses](https://leetcode.com/problems/restore-ip-addresses/)
- [131. Palindrome Partitioning](https://leetcode.com/problems/palindrome-partitioning/)
- [980. Unique Paths III](https://leetcode.com/problems/unique-paths-iii/)

---

출처: [Leetcode](https://leetcode.com/explore/interview/card/leetcodes-interview-crash-course-data-structures-and-algorithms/711/backtracking/4537/)

<!--

{: .prompt-general }

-->