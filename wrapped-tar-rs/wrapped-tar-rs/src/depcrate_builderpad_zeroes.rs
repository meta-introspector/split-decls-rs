// Generated macro for pad_zeroes (function)
macro_rules! Depcrate_builderpad_zeroes {
() => {
// Module: crate::builder
// Provides: {"pad_zeroes"}
// Dependencies: {}
fn pad_zeroes (dst : & mut dyn Write , len : u64) -> io :: Result < () > { let buf = [0 ; BLOCK_SIZE as usize] ; let remaining = BLOCK_SIZE - (len % BLOCK_SIZE) ; if remaining < BLOCK_SIZE { dst . write_all (& buf [.. remaining as usize]) ? ; } Ok (()) }
};
}
