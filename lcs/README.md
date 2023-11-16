# Longest Common Subsequence

## Problem Description

Given two strings, find the length of the longest subsequence present in both of them. Both strings are in uppercase Latin alphabets.

## Solution Approach

This problem can be efficiently solved using dynamic programming. We'll create a dynamic programming matrix to compute and store the lengths of the Longest Common Subsequence (LCS) for substrings of the input strings.

Let `s1` and `s2` be the two input strings of lengths `len1` and `len2`, respectively.

- We create a matrix `dp` of size `(len1 + 1) x (len2 + 1)` to store LCS lengths for substrings.
- We iterate through the characters of both strings and fill the matrix using the following rules:
  - If `s1[i] == s2[j]`, we increment `dp[i+1][j+1]` by 1, indicating an extended LCS.
  - If `s1[i] != s2[j]`, we take the maximum of `dp[i][j+1]` and `dp[i+1][j]`, indicating that we don't include the current character in the LCS.
- The final answer is stored in `dp[len1][len2]`, representing the length of the longest common subsequence.

## Time and Space complexity

The procedure cost in time is dominated by the construction of the dynamic programming matrix, which has len1 rows and len2 columns, giving us a complexity of O(len1*len2).
The same matrix, which is the only auxiliary structure that we use, gives us a space complexity of O(len1*len2).

## Input

The two vectors on which to compute the LCS (already provided in the main method):

## Output

The length of the longest common subsequence, along with a few test results.