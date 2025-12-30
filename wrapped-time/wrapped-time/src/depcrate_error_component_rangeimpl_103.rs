// Generated macro for impl_103 (impl)
macro_rules! Depcrate_error_component_rangeimpl_103 {
() => {
// Module: crate::error::component_range
// Provides: {"impl_103"}
// Dependencies: {}
impl ComponentRange { # [doc = " Obtain the name of the component whose value was out of range."] # [inline] pub const fn name (self) -> & 'static str { self . name } # [doc = " Whether the value's permitted range is conditional, i.e. whether an input with this"] # [doc = " value could have succeeded if the values of other components were different."] # [inline] pub const fn is_conditional (self) -> bool { self . conditional_message . is_some () } }
};
}
