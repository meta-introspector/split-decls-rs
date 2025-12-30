// Generated macro for SET_COD_SERVICE (function)
macro_rules! Depcrate_shared_bthdefSET_COD_SERVICE {
() => {
// Module: crate::shared::bthdef
// Provides: {"SET_COD_SERVICE"}
// Dependencies: {}
# [inline] pub fn SET_COD_SERVICE (cod : BTH_COD , service : u16) -> BTH_COD { (cod & ! COD_SERVICE_MASK) | ((service as u32) << COD_SERVICE_BIT_OFFSET) }
};
}
