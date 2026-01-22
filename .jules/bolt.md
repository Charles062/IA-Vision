## 2024-05-23 - Lazy Evaluation Win
**Learning:** `Array.map().slice()` iterates the entire array before slicing. `Array.slice().map()` only iterates the needed elements.
**Action:** When filtering or limiting a mapped array, always slice/filter BEFORE mapping if the map operation is expensive or the array is large.
