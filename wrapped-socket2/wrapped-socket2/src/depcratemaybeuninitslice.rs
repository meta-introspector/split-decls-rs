// Generated macro for MaybeUninitSlice (struct)
macro_rules! DepcrateMaybeUninitSlice {
() => {
// Module: crate
// Provides: {"MaybeUninitSlice"}
// Dependencies: {}
# [doc = " A version of [`IoSliceMut`] that allows the buffer to be uninitialised."] # [doc = ""] # [doc = " [`IoSliceMut`]: std::io::IoSliceMut"] # [repr (transparent)] pub struct MaybeUninitSlice < 'a > (sys :: MaybeUninitSlice < 'a >) ;
};
}
