// Generated macro for sha1_digest_round_x4 (function)
macro_rules! Depcratesha1_digest_round_x4 {
() => {
// Module: crate
// Provides: {"sha1_digest_round_x4"}
// Dependencies: {}
# [doc = " Emulates `llvm.x86.sha1rnds4` intrinsic."] # [doc = " Performs 4 rounds of the message block digest."] fn sha1_digest_round_x4 (abcd : u32x4 , work : u32x4 , i : i8) -> u32x4 { const K0V : u32x4 = u32x4 (K0 , K0 , K0 , K0) ; const K1V : u32x4 = u32x4 (K1 , K1 , K1 , K1) ; const K2V : u32x4 = u32x4 (K2 , K2 , K2 , K2) ; const K3V : u32x4 = u32x4 (K3 , K3 , K3 , K3) ; match i { 0 => sha1rnds4c (abcd , work + K0V) , 1 => sha1rnds4p (abcd , work + K1V) , 2 => sha1rnds4m (abcd , work + K2V) , 3 => sha1rnds4p (abcd , work + K3V) , _ => panic ! ("unknown icosaround index") , } }
};
}
