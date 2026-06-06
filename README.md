# Combination Sum II — Backtracking Visualizer

A high-performance Rust implementation of the **Combination Sum II** problem on LeetCode. This solution uses a highly efficient **Include/Exclude (Pick/Don't-Pick) Binary Decision Tree** model with structural duplicate pruning.

## 🚀 The Core Algorithm

The problem requires finding all unique combinations in a collection of candidate numbers (`arr`) where the candidate numbers sum up to a specific `target`. Each number in `arr` may only be used **once** per combination.

This implementation works using two core principles:
1. **First Sort, Then Find:** The input slice must be sorted in ascending order. This groups duplicate values together and unlocks optimal tree pruning.
2. **Binary Decision Branches:** At each recursive step, the algorithm branches into two choices:
   * **Branch 1 (Include):** Add the current element to the stack and move to the next index.
   * **Branch 2 (Exclude):** Skip the current element entirely, skipping over all identical duplicate elements to prevent duplicate combinations.

---

## 📊 Visualizing the Execution Tree

Given an input array `arr = [1, 1, 2]` and `target = 3`, the algorithm builds a binary state-space decision tree. 

* `idx` tracks our position in the array.
* `total` tracks the running sum.
* `cur_combo` tracks our current stack.

```text
                           [idx: 0, total: 0, combo: []]
                                     /       \
                                    /         \
                            INCLUDE            EXCLUDE
                              /                 \
                             /                   \
               [idx: 1, total: 1, [1]]          [idx: 2, total: 0, []] 
                     /        \                     /       \   (Skipped duplicate 1)
                    /          \                   /         \
            INCLUDE             EXCLUDE    INCLUDE            EXCLUDE
              /                   \          /                   \
[idx: 2, total: 2, [1,1]]   [idx: 2, total: 1, [1]]  ...         ...
     /        \
 INCLUDE    EXCLUDE
   /            \
(Over Target)  [idx: 3, total: 2, [1,1]] ❌ (End of Array, sum != target)
