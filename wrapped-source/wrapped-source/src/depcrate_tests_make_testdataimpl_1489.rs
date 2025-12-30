// Generated macro for impl_1489 (impl)
macro_rules! Depcrate_tests_make_testdataimpl_1489 {
() => {
// Module: crate::tests::make_testdata
// Provides: {"impl_1489"}
// Dependencies: {}
unsafe impl GlobalAlloc for MeasuringAllocator { unsafe fn alloc (& self , layout : Layout) -> * mut u8 { if Self :: ACTIVE . with (| f | f . get ()) { Self :: TOTAL_ALLOCATED . with (| c | c . set (c . get () + layout . size () as u64)) ; } System . alloc (layout) } unsafe fn dealloc (& self , ptr : * mut u8 , layout : Layout) { if Self :: ACTIVE . with (| f | f . get ()) { Self :: TOTAL_DEALLOCATED . with (| c | c . set (c . get () + layout . size () as u64)) ; } System . dealloc (ptr , layout) } }
};
}
