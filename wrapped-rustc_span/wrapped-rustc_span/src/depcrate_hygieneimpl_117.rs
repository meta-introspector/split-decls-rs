// Generated macro for impl_117 (impl)
macro_rules! Depcrate_hygieneimpl_117 {
() => {
// Module: crate::hygiene
// Provides: {"impl_117"}
// Dependencies: {}
impl MacroKind { pub fn descr (self) -> & 'static str { match self { MacroKind :: Bang => "macro" , MacroKind :: Attr => "attribute macro" , MacroKind :: Derive => "derive macro" , } } pub fn descr_expected (self) -> & 'static str { match self { MacroKind :: Attr => "attribute" , _ => self . descr () , } } pub fn article (self) -> & 'static str { match self { MacroKind :: Attr => "an" , _ => "a" , } } }
};
}
