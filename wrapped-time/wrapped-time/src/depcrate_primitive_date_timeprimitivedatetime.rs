// Generated macro for PrimitiveDateTime (struct)
macro_rules! Depcrate_primitive_date_timePrimitiveDateTime {
() => {
// Module: crate::primitive_date_time
// Provides: {"PrimitiveDateTime"}
// Dependencies: {}
# [doc = " Combined date and time."] # [derive (Clone , Copy , Eq)] # [cfg_attr (not (docsrs) , repr (C))] pub struct PrimitiveDateTime { # [cfg (target_endian = "little")] time : Time , # [cfg (target_endian = "little")] date : Date , # [cfg (target_endian = "big")] date : Date , # [cfg (target_endian = "big")] time : Time , }
};
}
