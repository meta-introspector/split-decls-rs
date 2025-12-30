// Generated macro for HeaderSlice (struct)
macro_rules! Depcrate_headerHeaderSlice {
() => {
// Module: crate::header
// Provides: {"HeaderSlice"}
// Dependencies: {}
# [doc = " Structure to allow Arc-managing some fixed-sized data and a variably-sized"] # [doc = " slice in a single allocation."] # [derive (Debug , Copy , Clone , Eq , PartialEq , Hash , PartialOrd , Ord)] # [repr (C)] pub struct HeaderSlice < H , T : ? Sized > { # [doc = " The fixed-sized data."] pub header : H , # [doc = " The dynamically-sized data."] pub slice : T , }
};
}
