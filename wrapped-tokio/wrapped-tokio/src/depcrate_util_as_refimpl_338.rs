// Generated macro for impl_338 (impl)
macro_rules! Depcrate_util_as_refimpl_338 {
() => {
// Module: crate::util::as_ref
// Provides: {"impl_338"}
// Dependencies: {}
impl AsRef < [u8] > for OwnedBuf { fn as_ref (& self) -> & [u8] { match self { Self :: Vec (vec) => vec , # [cfg (feature = "io-util")] Self :: Bytes (bytes) => bytes , } } }
};
}
