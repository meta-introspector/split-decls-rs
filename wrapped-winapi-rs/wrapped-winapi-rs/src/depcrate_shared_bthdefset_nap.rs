// Generated macro for SET_NAP (function)
macro_rules! Depcrate_shared_bthdefSET_NAP {
() => {
// Module: crate::shared::bthdef
// Provides: {"SET_NAP"}
// Dependencies: {}
# [inline] pub fn SET_NAP (nap : u16) -> BTH_ADDR { (nap as u64) << NAP_BIT_OFFSET }
};
}
