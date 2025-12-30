// Generated macro for test_hash (function)
macro_rules! Depcrate_output_teststest_hash {
() => {
// Module: crate::output_tests
// Provides: {"test_hash"}
// Dependencies: {}
# [test] fn test_hash () { test ! { ByteHash { struct Foo < T : Clone > (T) where Self : Sized ; } expands to { # [allow (deprecated)] # [automatically_derived] impl < T : Clone > :: zerocopy :: util :: macro_util :: core_reexport :: hash :: Hash for Foo < T > where Self : :: zerocopy :: IntoBytes + :: zerocopy :: Immutable , Self : Sized , { fn hash < H > (& self , state : & mut H) where H : :: zerocopy :: util :: macro_util :: core_reexport :: hash :: Hasher , { :: zerocopy :: util :: macro_util :: core_reexport :: hash :: Hasher :: write (state , :: zerocopy :: IntoBytes :: as_bytes (self)) } fn hash_slice < H > (data : & [Self] , state : & mut H) where H : :: zerocopy :: util :: macro_util :: core_reexport :: hash :: Hasher , { :: zerocopy :: util :: macro_util :: core_reexport :: hash :: Hasher :: write (state , :: zerocopy :: IntoBytes :: as_bytes (data)) } } } no_build } }
};
}
