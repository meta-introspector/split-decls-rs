// Generated macro for SliceExt (trait)
macro_rules! Depcrate_tendrilSliceExt {
() => {
// Module: crate::tendril
// Provides: {"SliceExt"}
// Dependencies: {}
# [doc = " `Tendril`-related methods for Rust slices."] pub trait SliceExt < F > : fmt :: Slice where F : fmt :: SliceFormat < Slice = Self > { # [doc = " Make a `Tendril` from this slice."] # [inline] fn to_tendril (& self) -> Tendril < F > { Tendril :: from_slice (self) } }
};
}
