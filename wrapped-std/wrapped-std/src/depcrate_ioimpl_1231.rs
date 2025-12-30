// Generated macro for impl_1231 (impl)
macro_rules! Depcrate_ioimpl_1231 {
() => {
// Module: crate::io
// Provides: {"impl_1231"}
// Dependencies: {}
# [stable (feature = "iovec" , since = "1.36.0")] impl < 'a > fmt :: Debug for IoSliceMut < 'a > { fn fmt (& self , fmt : & mut fmt :: Formatter < '_ >) -> fmt :: Result { fmt :: Debug :: fmt (self . 0 . as_slice () , fmt) } }
};
}
