## 2025-10-26 - Static Allocation for OCR Config
**Learning:** `rusty-tesseract::Args` involves allocating a `HashMap` and Strings. In a high-frequency loop (5s), re-allocating this configuration is wasteful.
**Action:** Use `std::sync::OnceLock` to initialize invariant configuration structures once and reuse them, especially when passed by reference to library functions.
