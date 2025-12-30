// Generated macro for test_into_bytes_enum (function)
macro_rules! Depcrate_output_teststest_into_bytes_enum {
() => {
// Module: crate::output_tests
// Provides: {"test_into_bytes_enum"}
// Dependencies: {}
# [test] fn test_into_bytes_enum () { macro_rules ! test_repr { ($ (# [$ attr : meta]) *) => { $ (test ! { IntoBytes { # [$ attr] enum Foo { Bar , } } expands to { # [allow (deprecated)] # [automatically_derived] unsafe impl :: zerocopy :: IntoBytes for Foo { fn only_derive_is_allowed_to_implement_this_trait () { } } } no_build }) * } ; } test_repr ! { # [repr (C)] # [repr (u8)] # [repr (u16)] # [repr (u32)] # [repr (u64)] # [repr (u128)] # [repr (usize)] # [repr (i8)] # [repr (i16)] # [repr (i32)] # [repr (i64)] # [repr (i128)] # [repr (isize)] } }
};
}
