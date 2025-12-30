// Generated macro for impl_1238 (impl)
macro_rules! Depcrate_ioimpl_1238 {
() => {
// Module: crate::io
// Provides: {"impl_1238"}
// Dependencies: {}
# [stable (feature = "iovec" , since = "1.36.0")] impl < 'a > fmt :: Debug for IoSlice < 'a > { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (self . 0 . as_slice () , fmt) } }
};
}
