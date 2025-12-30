// Generated macro for deref (macro)
macro_rules! Depcrate_enumerablederef {
() => {
// Module: crate::enumerable
// Provides: {"deref"}
// Dependencies: {}
macro_rules ! deref { ($ ($ (# [$ attrs : meta]) * $ ty : ty ,) *) => { $ ($ (# [$ attrs]) * impl < T : ? Sized + Enumerable > Enumerable for $ ty { fn definition (& self) -> EnumDef <'_ > { T :: definition (&** self) } fn variant (& self) -> Variant <'_ > { T :: variant (&** self) } }) * } ; }
};
}
