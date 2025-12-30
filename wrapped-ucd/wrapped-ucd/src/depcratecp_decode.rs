// Generated macro for cp_decode (function)
macro_rules! Depcratecp_decode {
() => {
// Module: crate
// Provides: {"cp_decode"}
// Dependencies: {}
fn cp_decode ((c1 , c2 , c3) : (u8 , u8 , u8)) -> char { let c = (c1 as u32) * 65536 + (c2 as u32) * 256 + (c3 as u32) ; unsafe { char :: from_u32_unchecked (c) } }
};
}
