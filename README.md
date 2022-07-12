# Sort Algorithms
### 1. Quick Sort
```rust
  pub mod sort
  fn main() {
    let mut arr: [i32; 8] = [342, 3, 23, 34, 129, 1024, 3074, 523];
    sort::quick_sort(&mut arr);
  }
```
Result
```
  3 23 34 129 342 523 1024 3074
```
### 2. Bubble Sort
```rust
  pub mod sort
  fn main() {
    let mut arr: [i32; 8] = [342, 3, 23, 34, 129, 1024, 3074, 523];
    sort::bubble_sort(&mut arr);
  }
```
Result
```
  3 23 34 129 342 523 1024 3074
```
### 3. Insertion Sort
```rust
  pub mod sort
  fn main() {
    let mut arr: [i32; 8] = [342, 3, 23, 34, 129, 1024, 3074, 523];
    sort::insertion_sort(&mut arr);
  }
```
Result
```
  3 23 34 129 342 523 1024 3074
```
### 4. Heap Sort
```rust
  pub mod sort
  fn main() {
    let mut arr: [i32; 8] = [342, 3, 23, 34, 129, 1024, 3074, 523];
    sort::heap_sort(&mut arr);
  }
```
Result
```
  3 23 34 129 342 523 1024 3074
```
### 5. Selection Sort
```rust
  pub mod sort
  fn main() {
    let mut arr: [i32; 8] = [342, 3, 23, 34, 129, 1024, 3074, 523];
    sort::selection_sort(&mut arr);
  }
```
Result
```
  3 23 34 129 342 523 1024 3074
```