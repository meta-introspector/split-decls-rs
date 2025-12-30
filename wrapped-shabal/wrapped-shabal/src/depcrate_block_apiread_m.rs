// Generated macro for read_m (function)
macro_rules! Depcrate_block_apiread_m {
() => {
// Module: crate::block_api
// Provides: {"read_m"}
// Dependencies: {}
# [inline] fn read_m (input : & Block) -> M { let mut m = [Wrapping (0) ; 16] ; for (o , chunk) in m . iter_mut () . zip (input . chunks_exact (4)) { let a = chunk . try_into () . unwrap () ; * o = Wrapping (u32 :: from_le_bytes (a)) ; } m }
};
}
