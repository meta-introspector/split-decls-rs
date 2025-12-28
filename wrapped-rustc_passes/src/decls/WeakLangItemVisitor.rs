macro_rules! WeakLangItemVisitor {
    () => {
        struct WeakLangItemVisitor < 'a , 'tcx > { tcx : TyCtxt < 'tcx > , items : & 'a mut lang_items :: LanguageItems , }
    };
}

WeakLangItemVisitor!()