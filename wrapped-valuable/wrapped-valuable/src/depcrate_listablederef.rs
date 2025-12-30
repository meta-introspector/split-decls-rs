// Generated macro for deref (macro)
macro_rules! Depcrate_listablederef {
() => {
// Module: crate::listable
// Provides: {"deref"}
// Dependencies: {}
macro_rules ! deref { ($ ($ (# [$ attrs : meta]) * $ ty : ty ,) *) => { $ ($ (# [$ attrs]) * impl < T : ? Sized + Listable > Listable for $ ty { fn size_hint (& self) -> (usize , Option < usize >) { T :: size_hint (&** self) } }) * } ; }
};
}
