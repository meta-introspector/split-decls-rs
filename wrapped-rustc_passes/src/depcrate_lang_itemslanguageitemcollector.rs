// Generated macro for LanguageItemCollector (struct)
macro_rules! Depcrate_lang_itemsLanguageItemCollector {
() => {
// Module: crate::lang_items
// Provides: {"LanguageItemCollector"}
// Dependencies: {}
struct LanguageItemCollector < 'ast , 'tcx > { items : LanguageItems , tcx : TyCtxt < 'tcx > , resolver : & 'ast ResolverAstLowering , item_spans : FxHashMap < DefId , Span > , parent_item : Option < & 'ast ast :: Item > , }
};
}
