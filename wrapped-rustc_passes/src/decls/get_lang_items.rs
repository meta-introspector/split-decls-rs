macro_rules! deps {
    () => {
        LanguageItemCollector!();
    };
}

macro_rules! get_lang_items {
    () => {
        deps!();
        # [doc = " Traverses and collects all the lang items in all crates."] fn get_lang_items (tcx : TyCtxt < '_ > , () : ()) -> LanguageItems { let resolver = tcx . resolver_for_lowering () . borrow () ; let (resolver , krate) = & * resolver ; let mut collector = LanguageItemCollector :: new (tcx , resolver) ; for & cnum in tcx . used_crates (()) . iter () { for & (def_id , lang_item) in tcx . defined_lang_items (cnum) . iter () { collector . collect_item (lang_item , def_id , None) ; } } visit :: Visitor :: visit_crate (& mut collector , krate) ; weak_lang_items :: check_crate (tcx , & mut collector . items , krate) ; collector . items }
    };
}

get_lang_items!();