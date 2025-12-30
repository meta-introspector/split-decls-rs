// Generated macro for impl_24 (impl)
macro_rules! Depcrate_attr_quirkimpl_24 {
() => {
// Module: crate::attr_quirk
// Provides: {"impl_24"}
// Dependencies: {}
impl Quirk { # [doc = " Returns a `Style` with the quirk `self` enabled."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```rust"] # [doc = " use yansi::{Style, Quirk::Mask};"] # [doc = ""] # [doc = " static MASKED: Style = Mask.style();"] # [doc = " ```"] pub const fn style (self) -> Style { Style :: new () . quirk (self) } }
};
}
