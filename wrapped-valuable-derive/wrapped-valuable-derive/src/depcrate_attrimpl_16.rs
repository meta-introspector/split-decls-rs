// Generated macro for impl_16 (impl)
macro_rules! Depcrate_attrimpl_16 {
() => {
// Module: crate::attr
// Provides: {"impl_16"}
// Dependencies: {}
impl MetaStyle { pub (crate) fn format (self , name : & str) -> String { match self { MetaStyle :: Ident => name . to_owned () , MetaStyle :: List => format ! ("{}(...)" , name) , MetaStyle :: NameValue => format ! ("{} = ..." , name) , } } }
};
}
