// Generated macro for GET_SAP (function)
macro_rules! Depcrate_shared_bthdefGET_SAP {
() => {
// Module: crate::shared::bthdef
// Provides: {"GET_SAP"}
// Dependencies: {}
# [inline] pub fn GET_SAP (addr : BTH_ADDR) -> u32 { ((addr & SAP_MASK) >> SAP_BIT_OFFSET) as u32 }
};
}
