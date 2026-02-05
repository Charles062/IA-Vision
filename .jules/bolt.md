## 2024-05-23 - Lazy Evaluation Win
**Learning:** `Array.map().slice()` iterates the entire array before slicing. `Array.slice().map()` only iterates the needed elements.
**Action:** When filtering or limiting a mapped array, always slice/filter BEFORE mapping if the map operation is expensive or the array is large.

## 2025-05-23 - Static Initialization in Hot Loops
**Learning:** Frequently re-initializing immutable structs (like Tesseract Args) in hot loops causes unnecessary allocations. Rust 1.70+ `OnceLock` solves this elegantly.
**Action:** Identify structs created in loops. If they are constant, move them to a `static OnceLock` or `lazy_static`.
