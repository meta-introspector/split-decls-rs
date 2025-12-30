// Generated macro for impl_87 (impl)
macro_rules! Depcrate_hygieneimpl_87 {
() => {
// Module: crate::hygiene
// Provides: {"impl_87"}
// Dependencies: {}
impl SyntaxContextData { fn root () -> SyntaxContextData { SyntaxContextData { outer_expn : ExpnId :: root () , outer_transparency : Transparency :: Opaque , parent : SyntaxContext :: root () , opaque : SyntaxContext :: root () , opaque_and_semiopaque : SyntaxContext :: root () , dollar_crate_name : kw :: DollarCrate , } } fn key (& self) -> SyntaxContextKey { (self . parent , self . outer_expn , self . outer_transparency) } }
};
}
