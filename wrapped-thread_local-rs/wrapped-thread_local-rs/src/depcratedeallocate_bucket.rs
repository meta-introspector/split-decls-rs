// Generated macro for deallocate_bucket (function)
macro_rules! Depcratedeallocate_bucket {
() => {
// Module: crate
// Provides: {"deallocate_bucket"}
// Dependencies: {}
# [doc = " # Safety"] # [doc = " The caller must ensure that `bucket` was allocated from [allocate_bucket]"] # [doc = " with the same `size` parameter."] unsafe fn deallocate_bucket < T > (bucket : * mut Entry < T > , size : usize) { let slice = unsafe { std :: slice :: from_raw_parts_mut (bucket , size) } ; drop (unsafe { Box :: from_raw (slice) }) ; }
};
}
