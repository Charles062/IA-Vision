## 2024-05-23 - Lazy Evaluation Win
**Learning:** `Array.map().slice()` iterates the entire array before slicing. `Array.slice().map()` only iterates the needed elements.
**Action:** When filtering or limiting a mapped array, always slice/filter BEFORE mapping if the map operation is expensive or the array is large.

## 2024-05-24 - Efficient Static Initialization
**Learning:** Initializing complex structs (like Tesseract Args) in a hot loop creates unnecessary allocation overhead.
**Action:** Use `std::sync::OnceLock` for lazy, one-time initialization of static configuration objects in Rust.
