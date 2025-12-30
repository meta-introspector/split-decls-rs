// Generated macro for unsafe_transmute (macro)
macro_rules! Depcrate_macrosunsafe_transmute {
() => {
// Module: crate::macros
// Provides: {"unsafe_transmute"}
// Dependencies: {}
# [cfg (all (uuid_unstable , feature = "zerocopy"))] macro_rules ! unsafe_transmute (($ e : expr) => { zerocopy :: transmute ! ($ e) }) ;
};
}
