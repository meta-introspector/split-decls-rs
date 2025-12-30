// Generated macro for GET_COD_MAJOR (function)
macro_rules! Depcrate_shared_bthdefGET_COD_MAJOR {
() => {
// Module: crate::shared::bthdef
// Provides: {"GET_COD_MAJOR"}
// Dependencies: {}
# [inline] pub fn GET_COD_MAJOR (cod : BTH_COD) -> u8 { ((cod & COD_MAJOR_MASK) >> COD_MAJOR_BIT_OFFSET) as u8 }
};
}
