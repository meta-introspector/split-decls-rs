// Generated macro for deref (macro)
macro_rules! Depcrate_tuplablederef {
() => {
// Module: crate::tuplable
// Provides: {"deref"}
// Dependencies: {}
macro_rules ! deref { ($ ($ (# [$ attrs : meta]) * $ ty : ty ,) *) => { $ ($ (# [$ attrs]) * impl < T : ? Sized + Tuplable > Tuplable for $ ty { fn definition (& self) -> TupleDef { T :: definition (&** self) } }) * } ; }
};
}
