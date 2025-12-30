// Generated macro for SET_NAP_SAP (function)
macro_rules! Depcrate_shared_bthdefSET_NAP_SAP {
() => {
// Module: crate::shared::bthdef
// Provides: {"SET_NAP_SAP"}
// Dependencies: {}
# [inline] pub fn SET_NAP_SAP (nap : u16 , sap : u32) -> BTH_ADDR { SET_NAP (nap) | SET_SAP (sap) }
};
}
