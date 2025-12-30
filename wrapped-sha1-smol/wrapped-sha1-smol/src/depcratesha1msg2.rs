// Generated macro for sha1msg2 (function)
macro_rules! Depcratesha1msg2 {
() => {
// Module: crate
// Provides: {"sha1msg2"}
// Dependencies: {}
# [doc = " Emulates `llvm.x86.sha1msg2` intrinsic."] fn sha1msg2 (a : u32x4 , b : u32x4) -> u32x4 { let u32x4 (x0 , x1 , x2 , x3) = a ; let u32x4 (_ , w13 , w14 , w15) = b ; let w16 = (x0 ^ w13) . rotate_left (1) ; let w17 = (x1 ^ w14) . rotate_left (1) ; let w18 = (x2 ^ w15) . rotate_left (1) ; let w19 = (x3 ^ w16) . rotate_left (1) ; u32x4 (w16 , w17 , w18 , w19) }
};
}
