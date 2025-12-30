// Generated macro for read_u64v_le (function)
macro_rules! Depcrateread_u64v_le {
() => {
// Module: crate
// Provides: {"read_u64v_le"}
// Dependencies: {}
fn read_u64v_le (ns : & mut [u64] , buf : & [u8]) { for (c , n) in buf . chunks_exact (8) . zip (ns) { * n = u64 :: from_le_bytes (c . try_into () . unwrap ()) ; } }
};
}
