// Generated macro for impl_702 (impl)
macro_rules! Depcrate_itemsimpl_702 {
() => {
// Module: crate::items
// Provides: {"impl_702"}
// Dependencies: {}
impl < 'a > Item < 'a > { fn from_foreign_mod (fm : & 'a ast :: ForeignMod , span : Span , config : & Config) -> Item < 'a > { Item { safety : fm . safety , abi : format_extern (ast :: Extern :: from_abi (fm . abi , DUMMY_SP) , config . force_explicit_abi () ,) , vis : None , body : fm . items . iter () . map (| i | BodyElement :: ForeignItem (i)) . collect () , span , } } }
};
}
