// Generated macro for get (function)
macro_rules! Depcrate_low_level_channelget {
() => {
// Module: crate::low_level::channel
// Provides: {"get"}
// Dependencies: {}
fn get (n : u16 , idx : u16) -> u16 { (n >> (BITS * idx)) & MASK }
};
}
