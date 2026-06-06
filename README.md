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

# Deep Dive: Include/Exclude Backtracking Engine for Combination Sum II

A mathematical and architectural breakdown of a high-performance Rust implementation for the **Combination Sum II** problem. This engine utilizes a **State-Space Binary Decision Tree** (commonly known as the *Pick/Don't-Pick* approach) tightly coupled with linear pointer pruning.

---

## 🧭 Core Architectural Strategy

Unlike traditional backtracking models that iterate through an array using explicit `for` loops at every stack level, this design abstracts the problem into a strict **binary decision engine**. At any given element index, the computer is forced to evaluate exactly two distinct micro-states:

1. **The Include Path (Pick):** Push the current element into the working stack, accumulate its value, and step natively to the neighboring index ($idx + 1$).
2. **The Exclude Path (Don't Pick):** Completely bypass the current element pool by utilizing a look-ahead pointer (`temp`) to jump straight to the next *unique* integer value.

---

## 🏗️ Detailed Code Walkthrough & Mechanics

```rust
fn find_combo(
    arr: &[i32], 
    target: i32, 
    idx: usize, 
    total: i32, 
    combo: &mut Vec<Vec<i32>>, 
    cur_combo: &mut Vec<i32>
) {
    // ==========================================
    // 1. THE SUCCESS GATEWAY
    // ==========================================
    if total == target {
        // Rust Ownership Mechanic: 'cur_combo' is a shared mutable reference (&mut)
        // pointing to a single allocation on the heap. We must physically deep-copy
        // its contents using '.clone()' to preserve a snapshot of this winning state.
        combo.push(cur_combo.clone());
        return;
    }

    // ==========================================
    // 2. THE FAILURE / BOUNDS GATEWAY
    // ==========================================
    // Out-of-bounds index tracking or an explicit over-target calculation
    // halts execution, popping the current frame cleanly off the runtime stack.
    if idx >= arr.len() || total > target {
        return;
    }

    // ==========================================
    // 3. BRANCH A: THE "INCLUDE" DECISION
    // ==========================================
    cur_combo.push(arr[idx]);
    
    // Dive deeper into the left side of the tree, incrementing the index and total sum
    find_combo(arr, target, idx + 1, total + arr[idx], combo, cur_combo);
    
    // BACKTRACKING STEP: Once execution unwinds back to this frame, we must pop 
    // the value we just added to restore the heap vector's pristine parent state.
    cur_combo.pop();

    // ==========================================
    // 4. THE ANTI-DUPLICATE SKIPPING ENGINE
    // ==========================================
    let mut temp: usize = idx;

    // Because the source candidate array is SORTED, all identical values are 
    // clustered sequentially. We sweep 'temp' forward until it breaks through
    // to a completely fresh integer value.
    while arr[idx] == arr[temp] {
        temp += 1;

        if temp >= arr.len() {
            break;
        }
    }

    // ==========================================
    // 5. BRANCH B: THE "EXCLUDE" DECISION
    // ==========================================
    // Swing execution down the right side of the tree, passing the clean 'total'
    // sum alongside the newly computed unique index pointer ('temp').
    find_combo(arr, target, temp, total, combo, cur_combo);
}

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
