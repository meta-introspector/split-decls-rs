// Generated macro for impl_251 (impl)
macro_rules! Depcrateimpl_251 {
() => {
// Module: crate
// Provides: {"impl_251"}
// Dependencies: {}
impl Parse < SyntaxNode > { pub fn cast < N : AstNode > (mut self) -> Option < Parse < N > > { if N :: cast (self . syntax_node ()) . is_some () { Some (Parse { green : self . green . take () , errors : self . errors . take () , _ty : PhantomData }) } else { None } } }
};
}
