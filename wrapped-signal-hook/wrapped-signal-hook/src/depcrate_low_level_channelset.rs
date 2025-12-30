// Generated macro for set (function)
macro_rules! Depcrate_low_level_channelset {
() => {
// Module: crate::low_level::channel
// Provides: {"set"}
// Dependencies: {}
fn set (n : u16 , idx : u16 , v : u16) -> u16 { let v = v << (BITS * idx) ; let mask = MASK << (BITS * idx) ; (n & ! mask) | v }
};
}
