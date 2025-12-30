// Generated macro for FN (macro)
macro_rules! Depcrate_macrosFN {
() => {
// Module: crate::macros
// Provides: {"FN"}
// Dependencies: {}
macro_rules ! FN { (stdcall $ func : ident ($ ($ t : ty ,) *) -> $ ret : ty) => (pub type $ func = Option < unsafe extern "system" fn ($ ($ t ,) *) -> $ ret >;) ; (stdcall $ func : ident ($ ($ p : ident : $ t : ty ,) *) -> $ ret : ty) => (pub type $ func = Option < unsafe extern "system" fn ($ ($ p : $ t ,) *) -> $ ret >;) ; (cdecl $ func : ident ($ ($ t : ty ,) *) -> $ ret : ty) => (pub type $ func = Option < unsafe extern "C" fn ($ ($ t ,) *) -> $ ret >;) ; (cdecl $ func : ident ($ ($ p : ident : $ t : ty ,) *) -> $ ret : ty) => (pub type $ func = Option < unsafe extern "C" fn ($ ($ p : $ t ,) *) -> $ ret >;) ; }
};
}
