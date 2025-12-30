// Generated macro for size_ok (function)
macro_rules! Depcratesize_ok {
() => {
// Module: crate
// Provides: {"size_ok"}
// Dependencies: {}
# [doc = " Check data structure size, comparing strictly on 64-bit targets."] # [doc = ""] # [doc = " - On 32-bit targets, checks if `actual_size` is at most `expected_64_bit_size`."] # [doc = " - On 64-bit targets, checks if `actual_size` is exactly `expected_64_bit_size`."] # [doc = ""] # [doc = " This is for assertions about the size of data structures, when the goal is to keep them from"] # [doc = " growing too large even across breaking changes. Such assertions must always fail when data"] # [doc = " structures grow larger than they have ever been, for which `<=` is enough. But it also helps to"] # [doc = " know when they have shrunk unexpectedly. They may shrink, other changes may rely on the smaller"] # [doc = " size for acceptable performance, and then they may grow again to their earlier size."] # [doc = ""] # [doc = " The problem with `==` is that data structures are often smaller on 32-bit targets. This could"] # [doc = " be addressed by asserting separate exact 64-bit and 32-bit sizes. But sizes may also differ"] # [doc = " across 32-bit targets, due to ABI and layout/packing details. That can happen across 64-bit"] # [doc = " targets too, but it seems less common."] # [doc = ""] # [doc = " For those reasons, this function does a `==` on 64-bit targets, but a `<=` on 32-bit targets."] pub fn size_ok (actual_size : usize , expected_64_bit_size : usize) -> bool { # [cfg (target_pointer_width = "64")] return actual_size == expected_64_bit_size ; # [cfg (target_pointer_width = "32")] return actual_size <= expected_64_bit_size ; }
};
}
