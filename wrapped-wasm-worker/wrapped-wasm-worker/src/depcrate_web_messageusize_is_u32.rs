// Generated macro for usize_is_u32 (function)
macro_rules! Depcrate_web_messageusize_is_u32 {
() => {
// Module: crate::web::message
// Provides: {"usize_is_u32"}
// Dependencies: {}
# [doc = " We assume that we aren't targeting `wasm64`."] fn usize_is_u32 (value : usize) -> u32 { value . try_into () . expect ("found 64-bit Wasm") }
};
}
