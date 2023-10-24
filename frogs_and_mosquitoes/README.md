## Problem

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

## Execution

To run the code, follow the instructions in the provided `main.rs` file. You can modify the test cases in the `main` function to suit your specific input and check the results.

## Input

The program expects two vectors of tuples as input:

- The `frogs` vector represents the frogs with their positions and tongue lengths.
- The `mosquitoes` vector represents the mosquitoes with their positions and sizes.

## Output

The program will return a vector of tuples containing the number of mosquitoes eaten by each frog and the total tongue length after eating the mosquitoes.

## Examples

Here are some sample executions with their respective results:

### Example 1

```rust
let frogs = vec![(10, 2), (15, 0), (6, 1), (0, 1)];
let mosqs = vec![(110, 10), (1, 1), (6, 0), (15, 10), (14, 100), (12, 2)];
let out = vec![(3, 114), (1, 10), (1, 1), (1, 2)];
let res = eat_mosquitoes(frogs, mosqs);
