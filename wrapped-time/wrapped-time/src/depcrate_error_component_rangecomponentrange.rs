// Generated macro for ComponentRange (struct)
macro_rules! Depcrate_error_component_rangeComponentRange {
() => {
// Module: crate::error::component_range
// Provides: {"ComponentRange"}
// Dependencies: {}
# [doc = " An error type indicating that a component provided to a method was out of range, causing a"] # [doc = " failure."] # [derive (Debug , Clone , Copy , Eq)] pub struct ComponentRange { # [doc = " Name of the component."] pub (crate) name : & 'static str , # [doc = " Minimum allowed value, inclusive."] pub (crate) minimum : i64 , # [doc = " Maximum allowed value, inclusive."] pub (crate) maximum : i64 , # [doc = " Value that was provided."] pub (crate) value : i64 , # [doc = " The minimum and/or maximum value is conditional on the value of other"] # [doc = " parameters."] pub (crate) conditional_message : Option < & 'static str > , }
};
}
