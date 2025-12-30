// Generated macro for GET_NAP (function)
macro_rules! Depcrate_shared_bthdefGET_NAP {
() => {
// Module: crate::shared::bthdef
// Provides: {"GET_NAP"}
// Dependencies: {}
# [inline] pub fn GET_NAP (addr : BTH_ADDR) -> u16 { ((addr & NAP_MASK) >> NAP_BIT_OFFSET) as u16 }
};
}
