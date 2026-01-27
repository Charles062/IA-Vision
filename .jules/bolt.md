## 2024-05-23 - Lazy Evaluation Win
**Learning:** `Array.map().slice()` iterates the entire array before slicing. `Array.slice().map()` only iterates the needed elements.
**Action:** When filtering or limiting a mapped array, always slice/filter BEFORE mapping if the map operation is expensive or the array is large.

## 2025-10-21 - Static Config Allocation
**Learning:** Recreating configuration objects (like `rusty_tesseract::Args` which contains a HashMap) in a hot loop adds unnecessary allocation overhead.
**Action:** Use `std::sync::OnceLock` to cache static configuration objects and reuse them.
