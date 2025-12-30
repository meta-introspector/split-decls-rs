// Generated macro for deref (macro)
macro_rules! Depcrate_valuablederef {
() => {
// Module: crate::valuable
// Provides: {"deref"}
// Dependencies: {}
macro_rules ! deref { ($ ($ (# [$ attrs : meta]) * $ ty : ty ,) *) => { $ ($ (# [$ attrs]) * impl < T : ? Sized + Valuable > Valuable for $ ty { fn as_value (& self) -> Value <'_ > { T :: as_value (&** self) } fn visit (& self , visit : & mut dyn Visit) { T :: visit (&** self , visit) ; } }) * } ; }
};
}
