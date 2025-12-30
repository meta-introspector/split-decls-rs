// Generated macro for SET_SAP (function)
macro_rules! Depcrate_shared_bthdefSET_SAP {
() => {
// Module: crate::shared::bthdef
// Provides: {"SET_SAP"}
// Dependencies: {}
# [inline] pub fn SET_SAP (sap : u32) -> BTH_ADDR { (sap as u64) << SAP_BIT_OFFSET }
};
}
