// Generated macro for sha1_first_half (function)
macro_rules! Depcratesha1_first_half {
() => {
// Module: crate
// Provides: {"sha1_first_half"}
// Dependencies: {}
# [doc = " Emulates `llvm.x86.sha1nexte` intrinsic."] # [inline] fn sha1_first_half (abcd : u32x4 , msg : u32x4) -> u32x4 { sha1_first_add (sha1_first (abcd) . rotate_left (30) , msg) }
};
}
