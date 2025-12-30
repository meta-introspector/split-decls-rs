// Generated macro for GET_COD_LAN_ACCESS (function)
macro_rules! Depcrate_shared_bthdefGET_COD_LAN_ACCESS {
() => {
// Module: crate::shared::bthdef
// Provides: {"GET_COD_LAN_ACCESS"}
// Dependencies: {}
# [inline] pub fn GET_COD_LAN_ACCESS (cod : BTH_COD) -> u8 { ((cod & COD_LAN_ACCESS_MASK) >> COD_LAN_ACCESS_BIT_OFFSET) as u8 }
};
}
