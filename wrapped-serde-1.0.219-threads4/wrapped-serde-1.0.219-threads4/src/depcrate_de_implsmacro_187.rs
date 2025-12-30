// Generated macro for macro_187 (macro)
macro_rules! Depcrate_de_implsmacro_187 {
() => {
// Module: crate::de::impls
// Provides: {"macro_187"}
// Dependencies: {}
impl_deserialize_num ! { i128 , NonZeroI128 cfg (not (no_num_nonzero_signed)) , deserialize_i128 num_self ! (i128 : visit_i128) ; num_as_self ! (i8 : visit_i8 i16 : visit_i16 i32 : visit_i32 i64 : visit_i64) ; num_as_self ! (u8 : visit_u8 u16 : visit_u16 u32 : visit_u32 u64 : visit_u64) ; num_128 ! (u128 : visit_u128) ; }
};
}
