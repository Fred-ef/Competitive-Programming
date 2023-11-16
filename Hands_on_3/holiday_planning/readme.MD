# Holiday Planning

## Problem Description

As part of the problem, you are provided with the number of attractions you can visit for each of the D days, in each city. The number of cities is n. The objective is to identify the maximum number of attractions the tourist can visit. Note that you can only visit the attractions in the order provided by the guide (i.e. you cannot "cherry pick" the attractions of a specific day without the days before it). The expected time complexity of the solution is O(n * D^2).

## Solution

The solution employs a dynamic programming approach, drawing inspiration from the classic "Rod Cutting" problem.
In 'Rod Cutting,' to obtain optimal cuts, we gradually generate all possible cut combinations, saving them in a dynamic programming array to avoid recomputing the same results. This follows the recurrence relation:
- cut[i] = { 0 if i==0; max (for k in [1, i]) {V_k + cut[i-k]} else}
where V_k is the value of the piece of rope of length k.
For each length from 0 to i, we evaluate what we would get with its cut and the best cut on the remaining (i-k) length, choosing the best evaluation.

We do something very similar for our problem, taking the necessary differences into account: as we have both days and cities, our problem will be 'bidimensional,' instead of 'unidimensional'.
Let's first identify the structure of subproblems: to find the solution for D days and n cities, we will base it on the solution for (D-1) days and (n-1) cities, and so on until the base case where we have 0 days or 0 cities (both will obviously return zero). What we want to do is build the scenarios gradually, evaluating all relevant combinations of choosing attractions by adding step by step either more cities (calculating combinations with a larger pool of attractions to choose from) or more days (calculating longer combinations, considering more total attractions), and storing these calculations in a matrix so as not to have to recompute them for the future, keeping them ready for use.

There's a particular aspect to note: in each combination, attractions are summed together. Therefore, in all combinations where you stay more days in a given city, the sum of visitable attractions there will be made. Based on the fact that attractions need to be visited in the same order every time, instead of computing this sum each time (in O(D) steps), we can precompute a prefix sum for each city, indicating, in the i-th position, the total number of visitable attractions while staying in that city for i days.
We save these sums in a "prefix-sum matrix" [i][j], where i denotes the city, and j denotes the number of visitable attractions in i days in city j.
We are now ready to formulate our recurrence:
- M[d][i] = {0 if d==0 or i==0; max (for k in [0, d]) {pre[i][k] + M[d-k][i-1]}}
For each number of days from 0 to d, we evaluate what we would get by staying in the i-th city for that number of days and staying in the other (i-1) cities for the rest of the (d-k) days, choosing the best evaluation. The solution to the problem for D days and n cities will be at position M[D][n] in the way the dynamic programming matrix is constructed.

### Step-by-Step Description:

1. **Prefix Sum Matrix:**
   - Initialize a matrix for prefix sums, storing the cumulative attractions for each city and day.

2. **Dynamic Programming Matrix:**
   - Initialize a dynamic programming matrix to store the maximum attractions for each combination of days and cities.

3. **Fill Dynamic Programming Matrix:**
   - Iterate over days and cities, computing the maximum attractions based on previous results.
   - Utilize the prefix sum matrix to efficiently calculate the number attraction.

4. **Result:**
   - The solution for the full holiday is in the last cell of the dynamic programming matrix.

## Time and Space Complexity

For each of the D*n cells in the matrix we examine from a maximum of D configurations, each generated in constant time thanks to prefix arrays. The resulting time complexity is thus O(nD^2).
As supporting structures, we use the prefix-sum matrix, which requires O(n*D) space, and the dynamic programming matrix of size n*D, which also requires O(n*D) space, for a total space complexity of O(n*D).

## Input

n+1 lines:
- The first line contains n and D
- The following n lines each contain D different integer values and describe the itineraries 'I'.

## Output

The maximum number of attractions that you can visit.