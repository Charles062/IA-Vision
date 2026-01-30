## 2024-05-23 - Lazy Evaluation Win
**Learning:** `Array.map().slice()` iterates the entire array before slicing. `Array.slice().map()` only iterates the needed elements.
**Action:** When filtering or limiting a mapped array, always slice/filter BEFORE mapping if the map operation is expensive or the array is large.

## 2024-10-26 - Static Initialization Win
**Learning:** Initializing complex configuration structs (like `HashMap`s) inside a tight loop causes unnecessary allocation churn.
**Action:** Use `std::sync::OnceLock` (or `lazy_static`) to cache constant configurations globally.
