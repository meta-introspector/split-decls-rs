// Generated macro for impl_47 (impl)
macro_rules! Depcrateimpl_47 {
() => {
// Module: crate
// Provides: {"impl_47"}
// Dependencies: {}
impl < T : Send > Drop for ThreadLocal < T > { fn drop (& mut self) { for (i , bucket) in self . buckets . iter_mut () . enumerate () { let bucket_ptr = * bucket . get_mut () ; let this_bucket_size = 1 << i ; if bucket_ptr . is_null () { continue ; } unsafe { deallocate_bucket (bucket_ptr , this_bucket_size) } ; } } }
};
}
