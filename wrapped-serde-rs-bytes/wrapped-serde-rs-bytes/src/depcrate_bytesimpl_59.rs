// Generated macro for impl_59 (impl)
macro_rules! Depcrate_bytesimpl_59 {
() => {
// Module: crate::bytes
// Provides: {"impl_59"}
// Dependencies: {}
# [cfg (any (feature = "std" , feature = "alloc"))] impl Default for Box < Bytes > { fn default () -> Self { ByteBuf :: new () . into_boxed_bytes () } }
};
}
