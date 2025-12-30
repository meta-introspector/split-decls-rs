// Generated macro for write_u64v_le (function)
macro_rules! Depcratewrite_u64v_le {
() => {
// Module: crate
// Provides: {"write_u64v_le"}
// Dependencies: {}
fn write_u64v_le (buf : & mut [u8] , ns : & [u64]) { for (c , n) in buf . chunks_exact_mut (8) . zip (ns) { c . copy_from_slice (& n . to_le_bytes ()) ; } }
};
}
