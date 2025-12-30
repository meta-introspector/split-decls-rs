// Generated macro for GET_COD_LAN_MINOR (function)
macro_rules! Depcrate_shared_bthdefGET_COD_LAN_MINOR {
() => {
// Module: crate::shared::bthdef
// Provides: {"GET_COD_LAN_MINOR"}
// Dependencies: {}
# [inline] pub fn GET_COD_LAN_MINOR (cod : BTH_COD) -> u8 { ((cod & COD_LAN_MINOR_MASK) >> COD_MINOR_BIT_OFFSET) as u8 }
};
}
