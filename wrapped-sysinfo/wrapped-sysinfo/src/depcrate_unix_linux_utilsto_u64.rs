// Generated macro for to_u64 (function)
macro_rules! Depcrate_unix_linux_utilsto_u64 {
() => {
// Module: crate::unix::linux::utils
// Provides: {"to_u64"}
// Dependencies: {}
# [cfg (feature = "system")] pub (crate) fn to_u64 (v : & [u8]) -> u64 { let mut x = 0 ; for c in v { x *= 10 ; x += u64 :: from (c - b'0') ; } x }
};
}
