// Generated macro for GET_COD_FORMAT (function)
macro_rules! Depcrate_shared_bthdefGET_COD_FORMAT {
() => {
// Module: crate::shared::bthdef
// Provides: {"GET_COD_FORMAT"}
// Dependencies: {}
# [inline] pub fn GET_COD_FORMAT (cod : BTH_COD) -> u8 { ((cod & COD_FORMAT_MASK) >> COD_FORMAT_BIT_OFFSET) as u8 }
};
}
