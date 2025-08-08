# Renan's Nathan Sort

The main objective of this repository is to improve my programming skills and apply the knowledge gained in the Algorithms Analysis and Design course. Furthermore, it aims to compare different sorting algorithms and test them in various scenarios.

## Quick start

Build and run (example):

```bash
# build with cargo
# open the cargo project of the chosed algorithm 
cargo check  # To check code
cargo run    # To run it without build

# To build it
cargo build 
./target/debug/my_project
```


## Algorithms:

### Insertion Sort

- Best Case: Θ(n)
  - Sorted input

- Worst Case: Θ(n²)
  - Reversed input

- In-place and Stable
- Easy to implementat
- When to use: input is almost sorted

### Selection Sort
- Best Case: Θ(n²)
  - Sorted input

- Worst Case: Θ(n²)
  - Reversed input

- In-place but not stable
- Easy to implement
- When to use: Someone must know when, but I don't

### Merge Sort
- Time complexity: Θ(nlogn) in all cases
  - Doesn't have best and worst case

- Not in-place but stable
- When to use: Time is critical regardless of the input, and you have a lot of memory to use

### Quick Sort
- Best Case: Θ(nlogn)
  - Balanced partitions - pivot is well chosen, it means you have slices whith nearly sizes in partitions

- Worst Case: Θ(n²)
  - Unbalanced partitions - pivot is poorly chosen 

- In-place but not stable
- When to use: Whenever you want, is quite easy to avoid the worst case and have a good execution
  - Some techniques to avoid the worst case are: Randomized Pivot, Median-of-Three, Median-of-Medians

### Counting Sort
- Pseudo-Linear: its time complexity Θ(n + k)
  - Its analysis depends k (maximum supported value in the input), because if you have k <= n, your algorithm is garanteed linear, but if you have k >= n², for example, you can have a bad time
- Not in-place but stable
- It's really limited, you just can use its original form with non negative integer (or values that can be mapped to it)
- When to use: When k isn't large relative to n (commonly (k ≤ n)

### Radix Sort
- Pseudo-Linear: it runs in time Θ(d(n + k))
  - Its analysis depends on k (the base) and d (number of digits you'll have using base k)

- Isn't in-place but is stable
- Also really limited, you just can use its in same cases of Counting Sort
- It's mainly Idea is to conserve Counting Sort assintotic time, but leading on k problem, applying Counting Sort across digits
- When to use: When d or k aren't big enough to make it bad

### Heap Sort
- Alway Θ(nlogn)
- In-place but not stable
- When to use: when you need garanteed execution time nlog and don't mind about stability

### Bubble Sort
- Best Case: Θ(n)
  - Sorted input

- Worst Case: Θ(n²)
  - Reversed input

- In-place and Stable
- When to use: When you want a nice and simple algotithm to have fun with, but more whith educational purposes

## Stable Algortihms:

The following algorithms are stable. This means that if there are equal values in the original list, their order will be preserved in the sorted list.

- ✔️ Insertion Sort
- ❌ Selection Sort
- ✔️ Merge Sort
- ❌ Quick Sort
- ✔️ Counting Sort
- ✔️ Radix Sort
- ❌ Heap Sort
- ✔️ Bubble Sort

## In-place Algorithms:

For this analysis, was proposed that an algorithm was *not in-place* just in cases when it allocates aulixiary lists or arrays

- ✔️ Insertion Sort
- ✔️ Selection Sort
- ❌ Merge Sort
- ✔️ Quick Sort
- ❌ Counting Sort
- ❌ Radix Sort
- ✔️ Heap Sort
- ✔️ Bubble Sort

Considering a more complex Space Analyse, the only one that are in-place are Insertion Sort, Selection Sort and Bubble Sort

## Working Area:

- *Data Structure*
- *Sort Algorithms Benchmark*

## Coming soon:

- *Basic Algorithms*
- *Some heuristics*

## License

This project is released under the MIT License. See `LICENSE` for details.
