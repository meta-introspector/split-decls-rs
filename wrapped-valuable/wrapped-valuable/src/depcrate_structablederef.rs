// Generated macro for deref (macro)
macro_rules! Depcrate_structablederef {
() => {
// Module: crate::structable
// Provides: {"deref"}
// Dependencies: {}
macro_rules ! deref { ($ ($ (# [$ attrs : meta]) * $ ty : ty ,) *) => { $ ($ (# [$ attrs]) * impl < T : ? Sized + Structable > Structable for $ ty { fn definition (& self) -> StructDef <'_ > { T :: definition (&** self) } }) * } ; }
};
}
