# Xmas Lights

## Problem Description

In a famous street, there are n houses, each decorated with Christmas lights of the owner’s favorite color. The possible colors are Red, White, and Green, represented by the characters 'R', 'W', and 'G'. The colors are listed in an array H[1, n], where H[i] is the color of the ith house on the street.

The goal is to compute the number of patriotic selections of three houses (1 ≤ i < j < p ≤ n) such that H[i] = Red, H[j] = White, and H[p] = Green. Note that houses are not required to be consecutive.

The problem introduces an additional challenge: some houses can have an unassigned color, represented by 'X', which can be set to any of the three colors. Fixing these unassigned colors results in 3^k possible different street colorings, where k is the number of houses with unassigned colors. The task is to compute the number of patriotic selections for all possible colorings efficiently using dynamic programming.

## Solution - Without Unassigned Colors ('X')

Let's begin with the variant without 'X'. The solution is pretty simple. We use three counters:

- count_R: Represents the number of 'R' encountered so far.
- count_RW: Represents the number of sequences "RW" encountered.
- count_RWG: Represents the number of complete sequences "RWG" encountered.
At each step:
- When encountering 'R', increment count_R by 1.
- When encountering 'W', 'W' can pair with any of the previously seen 'R'; hence, increment count_RW by the value of count_R.
- When encountering 'G', 'G' can be appended to any "WR" sequence seen so far; hence, increment count_RWG by the value of count_RW.
This provides a linear-time and constant-space solution to the problem.

## Solution - With Unassigned Colors ('X')

We'll scan the input string S element by element, keeping a counter count_x to represent the number of 'X' encountered so far, and three additional counters:

- count_r: Represents the number of sequences formed by the single character 'R'.
- count_rw: Represents the number of sequences of two characters 'RW' encountered.
- count_rwg: Represents the number of complete sequences 'RWG' encountered.

Let's now make a consideration this: when encountering an 'X', it's as if our sequence branches into three different sequences ('R', 'W', and 'G'). When another 'X' is encountered, each of the existing sequences ideally branches into three more, following the same principle. Instead of enumerating all 3^k possibilities, we keep track of their number by incrementing count_x each time 'X' is encountered. When 'R' is encountered, we can virtually add it to all tracked sequences, adding 3^count_x to count_r for each 'R' seen (i.e., 1 before 'X', 3 with one 'X', 9 with two 'X', etc.).

This approach eliminates the need to modify the procedure when encountering 'W' or 'G': since both count_rw and count_rwg are functions of count_r, they return the correct values accordingly.

One last thing to consider: when encountering an 'X', we must also consider having seen 'R', 'W', and 'G' in three different branches. Thus, 'R' adds to an existing branch, 'W' adds to another, and 'G' adds to yet another. We need to consider each of these to obtain the correct result. As mentioned, we maintain only one sequence, so when encountering an 'X', we need to "condense" the three branches into one, adding the three letters that 'X' represents ('R', 'W', and 'G') to triple the values of count_r, count_rw, and count_rwg.

We can now define a recurrence relation that follows this pattern ('k' is the number of 'X's encountered thus far):

- count_r[i] = {count_r[i-1]*3^k if S[i] == 'R'; 3*count_r[i-1]*3^k if S[i] == 'X'; count_r[i-1] else}
- count_rw[i] = {count_rw[i-1] + count_r[i-1] if S[i] == 'W'; 3*count_rw[i-1] + count_r[i-1] if S[i] == 'X'; count_rw[i-1] else;}
- count_rwg[i] = {count_rwg[i-1] + count_rw[i-1] if S[i] == 'G'; 3*count_rwg[i-1] + count_rw[i-1] if S[i] == 'X'; count_rwg[i-1] else}

## Space and Time complexity


The implementation follows this recurrence relation closely. Operationally, the presented solution scans the string only once, performing a constant number of additions (1 or 3) at each step, resulting in a time complexity of O(n). Additionally, it relies on only four variables to function, giving us a space complexity of O(1).

## Input

Multiple lines:
- The first line contains n, the number of houses
- The next line contains a character in { R, W, G, X } for each color of the n houses

## Output

The number of patriotic selections in the 3^k possible street colorings.