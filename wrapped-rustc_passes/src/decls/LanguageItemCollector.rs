macro_rules! LanguageItemCollector {
    () => {
        struct LanguageItemCollector < 'ast , 'tcx > { items : LanguageItems , tcx : TyCtxt < 'tcx > , resolver : & 'ast ResolverAstLowering , item_spans : FxHashMap < DefId , Span > , parent_item : Option < & 'ast ast :: Item > , }
    };
}

LanguageItemCollector!()