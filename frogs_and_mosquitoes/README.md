# Frogs and mosquitoes

## Problem description

You are given a group of frogs and a swarm of mosquitoes. Frogs can catch mosquitoes with their long, sticky tongues. The goal is to calculate how many insects each frog consumes while attempting to catch mosquitoes.

## Solution

The frogs are represented as triples: <x_i, t_i, i>, where "x_i" is the position of the i-th frog, "t_i" is the length of the i-th frog's tongue, and "i" is the index of the frog. Mosquitoes are represented as pairs: <p_j, b_j>, where "p_j" is the position of the mosquito, and "b_j" is its size. For simplicity, we will use <x_i, t_i> for frogs and <p_j, b_j> for mosquitoes without indices, but in the algorithm, the indices are essential!

Let's start with a solution overview:

1. Initialize data structures to represent frogs and mosquitoes.
2. Insert the frog segments (<x_i, t_i>) into the frog binary search tree (BST).
3. Perform an initial pass to remove overlaps among frog segments in the BST.
4. For each incoming mosquito <p_j, b_j>, perform the following:
   - Use a predecessor query on the frog BST to find the frog <x_i, t_i> at the immediate left of the mosquito.
   - If such a frog <x_i, t_i> is found and (x_i + t_i) >= p_j, it can eat the mosquito, and its tongue length t_i is updated to (t_i + b_j).
   - If no frog can eat the mosquito, insert it into the mosquito BST.
   - If a frog ate the mosquito, check for overlaps with subsequent frog segments, and resolve them as follows:
     - If there is a total overlap (x_i + t_i) >= (x_(i+1) + t_(i+1)), remove the frog <x_(i+1), t_(i+1)>.
     - If there is a partial overlap, adjust the frog segment to start just after the end of the previous frog segment (x_(i+1) = t_i + 1).
5. Check if a frog with an extended tongue can eat any remaining mosquitoes in the BST. Continue the process with the next mosquito.

In addition to tracking the frogs and mosquitoes, keep a vector called "result" whose items are couples <monsquitoes eaten, tongue length>. Its size is equal to the initial number of frogs. When a frog <x_i, t_i> eats a mosquito <p_j, b_j>, increment its "mosquitoes count" by 1 and its "tongue length" by b_j in the result vector.

## Time and Space complexity
The initial segments insertion and pre-processing costs O(nlgn).
The main procedure costs O(mlgn) for checking which frog can eat a given mosquito and update it (for each of the m mosquitoes), plus O(mlgm) to keep track of mosquitoes that have not yet been eaten. This gives us an overall time complexity of O((n+m)lg(n+m)).
As we only use the two trees, one cotaining at most all the segments and the other containing at most all the mosquitoes, and one vector containing all the frogs, the overall space complexity is O(n+m).

## Input

Two vectors of tuples (already provided in the main method):

- The `frogs` vector represents the frogs with their positions and tongue lengths.
- The `mosquitoes` vector represents the mosquitoes with their positions and sizes.

## Output

Each frog paired with its final tongue length, along with a few test results.