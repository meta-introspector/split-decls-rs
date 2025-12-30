// Generated macro for sha1msg1 (function)
macro_rules! Depcratesha1msg1 {
() => {
// Module: crate
// Provides: {"sha1msg1"}
// Dependencies: {}
# [doc = " Emulates `llvm.x86.sha1msg1` intrinsic."] fn sha1msg1 (a : u32x4 , b : u32x4) -> u32x4 { let u32x4 (_ , _ , w2 , w3) = a ; let u32x4 (w4 , w5 , _ , _) = b ; a ^ u32x4 (w2 , w3 , w4 , w5) }
};
}
