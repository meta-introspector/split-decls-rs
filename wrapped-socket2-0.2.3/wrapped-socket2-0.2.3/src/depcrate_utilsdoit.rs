// Generated macro for doit (macro)
macro_rules! Depcrate_utilsdoit {
() => {
// Module: crate::utils
// Provides: {"doit"}
// Dependencies: {}
macro_rules ! doit { ($ ($ t : ident) *) => ($ (impl NetInt for $ t { fn from_be (i : Self) -> Self { <$ t >:: from_be (i) } fn to_be (& self) -> Self { <$ t >:: to_be (* self) } }) *) }
};
}
