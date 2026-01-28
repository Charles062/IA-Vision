## 2024-05-23 - Lazy Evaluation Win
**Learning:** `Array.map().slice()` iterates the entire array before slicing. `Array.slice().map()` only iterates the needed elements.
**Action:** When filtering or limiting a mapped array, always slice/filter BEFORE mapping if the map operation is expensive or the array is large.

## 2025-02-23 - Rust Static Cache
**Learning:** Initializing complex structs like `rusty_tesseract::Args` (containing HashMap/String) in a loop causes unnecessary allocation churn. `std::sync::OnceLock` is the standard for thread-safe lazy globals.
**Action:** Use `static NAME: OnceLock<T> = OnceLock::new();` to cache invariant configuration objects used in hot loops.
