// Generated macro for deref (macro)
macro_rules! Depcrate_mappablederef {
() => {
// Module: crate::mappable
// Provides: {"deref"}
// Dependencies: {}
macro_rules ! deref { ($ ($ (# [$ attrs : meta]) * $ ty : ty ,) *) => { $ ($ (# [$ attrs]) * impl < T : ? Sized + Mappable > Mappable for $ ty { fn size_hint (& self) -> (usize , Option < usize >) { T :: size_hint (&** self) } }) * } ; }
};
}
