// Generated macro for impl_115 (impl)
macro_rules! Depcrate_hygieneimpl_115 {
() => {
// Module: crate::hygiene
// Provides: {"impl_115"}
// Dependencies: {}
impl ExpnKind { pub fn descr (& self) -> String { match * self { ExpnKind :: Root => kw :: PathRoot . to_string () , ExpnKind :: Macro (macro_kind , name) => match macro_kind { MacroKind :: Bang => format ! ("{name}!") , MacroKind :: Attr => format ! ("#[{name}]") , MacroKind :: Derive => format ! ("#[derive({name})]") , } , ExpnKind :: AstPass (kind) => kind . descr () . to_string () , ExpnKind :: Desugaring (kind) => format ! ("desugaring of {}" , kind . descr ()) , } } }
};
}
