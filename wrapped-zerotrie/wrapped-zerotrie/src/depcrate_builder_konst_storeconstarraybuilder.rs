// Generated macro for ConstArrayBuilder (struct)
macro_rules! Depcrate_builder_konst_storeConstArrayBuilder {
() => {
// Module: crate::builder::konst::store
// Provides: {"ConstArrayBuilder"}
// Dependencies: {}
# [doc = " A const-friendly mutable data structure backed by an array."] # [derive (Debug , Copy , Clone)] pub (crate) struct ConstArrayBuilder < const N : usize , T > { full_array : [T ; N] , start : usize , limit : usize , }
};
}
