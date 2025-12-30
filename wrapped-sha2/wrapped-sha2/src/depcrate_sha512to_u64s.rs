// Generated macro for to_u64s (function)
macro_rules! Depcrate_sha512to_u64s {
() => {
// Module: crate::sha512
// Provides: {"to_u64s"}
// Dependencies: {}
# [inline (always)] # [allow (dead_code)] fn to_u64s (block : & [u8 ; 128]) -> [u64 ; 16] { core :: array :: from_fn (| i | { let chunk = block [8 * i ..] [.. 8] . try_into () . unwrap () ; u64 :: from_be_bytes (chunk) }) }
};
}
