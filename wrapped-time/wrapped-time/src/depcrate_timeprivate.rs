// Generated macro for private (module)
macro_rules! Depcrate_timeprivate {
() => {
// Module: crate::time
// Provides: {"private"}
// Dependencies: {}
mod private { # [non_exhaustive] # [derive (Debug , Clone , Copy)] pub struct TimeMetadata { # [doc = " How many characters wide the formatted subsecond is."] pub (super) subsecond_width : u8 , # [doc = " The value to use when formatting the subsecond. Leading zeroes will be added as"] # [doc = " necessary."] pub (super) subsecond_value : u32 , } }
};
}
