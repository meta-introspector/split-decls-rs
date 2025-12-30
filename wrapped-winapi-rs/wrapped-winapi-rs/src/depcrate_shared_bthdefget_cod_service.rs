// Generated macro for GET_COD_SERVICE (function)
macro_rules! Depcrate_shared_bthdefGET_COD_SERVICE {
() => {
// Module: crate::shared::bthdef
// Provides: {"GET_COD_SERVICE"}
// Dependencies: {}
# [inline] pub fn GET_COD_SERVICE (cod : BTH_COD) -> u16 { ((cod & COD_SERVICE_MASK) >> COD_SERVICE_BIT_OFFSET) as u16 }
};
}
