// Generated macro for private (module)
macro_rules! Depcrate_dateprivate {
() => {
// Module: crate::date
// Provides: {"private"}
// Dependencies: {}
mod private { # [non_exhaustive] # [derive (Debug , Clone , Copy)] pub struct DateMetadata { # [doc = " The width of the year component, including the sign."] pub (super) year_width : u8 , # [doc = " Whether the sign should be displayed."] pub (super) display_sign : bool , pub (super) year : i32 , pub (super) month : u8 , pub (super) day : u8 , } }
};
}
