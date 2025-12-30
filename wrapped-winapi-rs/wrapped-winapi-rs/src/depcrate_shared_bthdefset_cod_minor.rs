// Generated macro for SET_COD_MINOR (function)
macro_rules! Depcrate_shared_bthdefSET_COD_MINOR {
() => {
// Module: crate::shared::bthdef
// Provides: {"SET_COD_MINOR"}
// Dependencies: {}
# [inline] pub fn SET_COD_MINOR (cod : BTH_COD , minor : u8) -> BTH_COD { (cod & ! COD_MINOR_MASK) | ((minor as u32) << COD_MINOR_BIT_OFFSET) }
};
}
