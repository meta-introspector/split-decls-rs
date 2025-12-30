// Generated macro for impl_56 (impl)
macro_rules! Depcrateimpl_56 {
() => {
// Module: crate
// Provides: {"impl_56"}
// Dependencies: {}
impl < S > SynToken < S > { fn token (& self) -> & SyntaxToken { match self { SynToken :: Ordinary (it) | SynToken :: Punct { token : it , offset : _ } => it , SynToken :: Leaf (_) => unreachable ! () , } } }
};
}
