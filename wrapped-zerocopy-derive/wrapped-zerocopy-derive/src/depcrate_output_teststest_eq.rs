// Generated macro for test_eq (function)
macro_rules! Depcrate_output_teststest_eq {
() => {
// Module: crate::output_tests
// Provides: {"test_eq"}
// Dependencies: {}
# [test] fn test_eq () { test ! { ByteEq { struct Foo < T : Clone > (T) where Self : Sized ; } expands to { # [allow (deprecated)] # [automatically_derived] impl < T : Clone > :: zerocopy :: util :: macro_util :: core_reexport :: cmp :: PartialEq for Foo < T > where Self : :: zerocopy :: IntoBytes + :: zerocopy :: Immutable , Self : Sized , { fn eq (& self , other : & Self) -> bool { :: zerocopy :: util :: macro_util :: core_reexport :: cmp :: PartialEq :: eq (:: zerocopy :: IntoBytes :: as_bytes (self) , :: zerocopy :: IntoBytes :: as_bytes (other) ,) } } # [allow (deprecated)] # [automatically_derived] impl < T : Clone > :: zerocopy :: util :: macro_util :: core_reexport :: cmp :: Eq for Foo < T > where Self : :: zerocopy :: IntoBytes + :: zerocopy :: Immutable , Self : Sized , { } } no_build } }
};
}
