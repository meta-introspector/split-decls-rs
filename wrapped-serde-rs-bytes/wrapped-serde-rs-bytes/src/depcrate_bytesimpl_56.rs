// Generated macro for impl_56 (impl)
macro_rules! Depcrate_bytesimpl_56 {
() => {
// Module: crate::bytes
// Provides: {"impl_56"}
// Dependencies: {}
# [cfg (any (feature = "std" , feature = "alloc"))] impl ToOwned for Bytes { type Owned = ByteBuf ; fn to_owned (& self) -> Self :: Owned { ByteBuf :: from (& self . bytes) } }
};
}
