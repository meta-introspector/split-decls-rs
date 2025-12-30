// Generated macro for SET_COD_MAJOR (function)
macro_rules! Depcrate_shared_bthdefSET_COD_MAJOR {
() => {
// Module: crate::shared::bthdef
// Provides: {"SET_COD_MAJOR"}
// Dependencies: {}
# [inline] pub fn SET_COD_MAJOR (cod : BTH_COD , major : u8) -> BTH_COD { (cod & ! COD_MAJOR_MASK) | ((major as u32) << COD_MAJOR_BIT_OFFSET) }
};
}
