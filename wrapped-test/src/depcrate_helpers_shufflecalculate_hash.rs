// Generated macro for calculate_hash (function)
macro_rules! Depcrate_helpers_shufflecalculate_hash {
() => {
// Module: crate::helpers::shuffle
// Provides: {"calculate_hash"}
// Dependencies: {}
fn calculate_hash < T : core :: hash :: Hash > (t : & T) -> u64 { let mut s = DefaultHasher :: new () ; t . hash (& mut s) ; s . finish () }
};
}
