// Generated macro for test_allocate_layout (function)
macro_rules! Depcratetest_allocate_layout {
() => {
// Module: crate
// Provides: {"test_allocate_layout"}
// Dependencies: {}
pub fn test_allocate_layout < A : Allocator > (alloc : A , layout : Layout) { if let Ok (ptr) = alloc . allocate (layout) { unsafe { alloc . deallocate (ptr . cast () , layout) } } }
};
}
