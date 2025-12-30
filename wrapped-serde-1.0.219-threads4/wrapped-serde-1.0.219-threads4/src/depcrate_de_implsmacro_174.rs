// Generated macro for macro_174 (macro)
macro_rules! Depcrate_de_implsmacro_174 {
() => {
// Module: crate::de::impls
// Provides: {"macro_174"}
// Dependencies: {}
impl_deserialize_num ! { i8 , NonZeroI8 cfg (not (no_num_nonzero_signed)) , deserialize_i8 num_self ! (i8 : visit_i8) ; int_to_int ! (i16 : visit_i16 i32 : visit_i32 i64 : visit_i64) ; uint_to_self ! (u8 : visit_u8 u16 : visit_u16 u32 : visit_u32 u64 : visit_u64) ; }
};
}
