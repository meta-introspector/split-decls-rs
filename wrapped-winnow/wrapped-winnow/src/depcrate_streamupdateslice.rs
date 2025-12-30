// Generated macro for UpdateSlice (trait)
macro_rules! Depcrate_streamUpdateSlice {
() => {
// Module: crate::stream
// Provides: {"UpdateSlice"}
// Dependencies: {}
# [doc = " Convert a `Stream` into an appropriate `Output` type"] pub trait UpdateSlice : Stream { # [doc = " Convert an `Output` type to be used as `Stream`"] fn update_slice (self , inner : Self :: Slice) -> Self ; }
};
}
