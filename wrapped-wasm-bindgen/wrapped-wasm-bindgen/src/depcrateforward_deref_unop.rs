// Generated macro for forward_deref_unop (macro)
macro_rules! Depcrateforward_deref_unop {
() => {
// Module: crate
// Provides: {"forward_deref_unop"}
// Dependencies: {}
macro_rules ! forward_deref_unop { (impl $ imp : ident , $ method : ident for $ t : ty) => { impl $ imp for $ t { type Output = <&'static $ t as $ imp >:: Output ; # [inline] fn $ method (self) -> <&'static $ t as $ imp >:: Output { $ imp ::$ method (& self) } } } ; }
};
}
