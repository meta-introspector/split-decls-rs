// Generated macro for volatile_set (function)
macro_rules! Depcratevolatile_set {
() => {
// Module: crate
// Provides: {"volatile_set"}
// Dependencies: {}
# [doc = " Perform a volatile `memset` operation which fills a slice with a value"] # [doc = ""] # [doc = " Safety:"] # [doc = " The memory pointed to by `dst` must be a single allocated object that is valid for `count`"] # [doc = " contiguous elements of `T`."] # [doc = " `count` must not be larger than an `isize`."] # [doc = " `dst` being offset by `size_of::<T> * count` bytes must not wrap around the address space."] # [doc = " Also `dst` must be properly aligned."] # [inline (always)] unsafe fn volatile_set < T : Copy + Sized > (dst : * mut T , src : T , count : usize) { for i in 0 .. count { let ptr = unsafe { dst . add (i) } ; unsafe { ptr :: write_volatile (ptr , src) } ; } }
};
}
