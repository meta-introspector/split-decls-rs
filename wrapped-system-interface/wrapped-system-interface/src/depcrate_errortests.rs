// Generated macro for tests (module)
macro_rules! Depcrate_errortests {
() => {
// Module: crate::error
// Provides: {"tests"}
// Dependencies: {}
# [cfg (test)] mod tests { use { super :: SystemError , num_traits :: FromPrimitive , strum :: IntoEnumIterator } ; # [test] fn test_system_error_from_primitive_exhaustive () { for variant in SystemError :: iter () { let variant_i64 = variant . clone () as i64 ; assert_eq ! (SystemError :: from_repr (variant_i64 as usize) , SystemError :: from_i64 (variant_i64)) ; assert_eq ! (SystemError :: from (variant_i64 as u64) , variant) ; } } }
};
}
