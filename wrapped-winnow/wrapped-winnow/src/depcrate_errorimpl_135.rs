// Generated macro for impl_135 (impl)
macro_rules! Depcrate_errorimpl_135 {
() => {
// Module: crate::error
// Provides: {"impl_135"}
// Dependencies: {}
impl < I , E > ParseError < I , E > { # [doc = " The [`Stream`] at the initial location when parsing started"] # [inline] pub fn input (& self) -> & I { & self . input } # [doc = " The location in [`ParseError::input`] where parsing failed"] # [doc = ""] # [doc = " To get the span for the `char` this points to, see [`ParseError::char_span`]."] # [doc = ""] # [doc = " <div class=\"warning\">"] # [doc = ""] # [doc = " **Note:** This is an offset, not an index, and may point to the end of input"] # [doc = " (`input.len()`) on eof errors."] # [doc = ""] # [doc = " </div>"] # [inline] pub fn offset (& self) -> usize { self . offset } # [doc = " The original [`ParserError`]"] # [inline] pub fn inner (& self) -> & E { & self . inner } # [doc = " The original [`ParserError`]"] # [inline] pub fn into_inner (self) -> E { self . inner } }
};
}
