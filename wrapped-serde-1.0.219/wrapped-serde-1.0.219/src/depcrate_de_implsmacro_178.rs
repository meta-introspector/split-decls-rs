// Generated macro for macro_178 (macro)
macro_rules! Depcrate_de_implsmacro_178 {
() => {
// Module: crate::de::impls
// Provides: {"macro_178"}
// Dependencies: {}
impl_deserialize_num ! { isize , NonZeroIsize cfg (not (no_num_nonzero_signed)) , deserialize_i64 num_as_self ! (i8 : visit_i8 i16 : visit_i16) ; int_to_int ! (i32 : visit_i32 i64 : visit_i64) ; uint_to_self ! (u8 : visit_u8 u16 : visit_u16 u32 : visit_u32 u64 : visit_u64) ; }
};
}
