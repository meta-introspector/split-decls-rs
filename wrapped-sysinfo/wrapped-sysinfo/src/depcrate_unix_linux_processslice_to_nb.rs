// Generated macro for slice_to_nb (function)
macro_rules! Depcrate_unix_linux_processslice_to_nb {
() => {
// Module: crate::unix::linux::process
// Provides: {"slice_to_nb"}
// Dependencies: {}
fn slice_to_nb (s : & [u8]) -> u64 { let mut nb : u64 = 0 ; for c in s { nb = nb * 10 + (c - b'0') as u64 ; } nb }
};
}
