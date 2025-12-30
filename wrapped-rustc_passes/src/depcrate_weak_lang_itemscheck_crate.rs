// Generated macro for check_crate (function)
macro_rules! Depcrate_weak_lang_itemscheck_crate {
() => {
// Module: crate::weak_lang_items
// Provides: {"check_crate"}
// Dependencies: {}
# [doc = " Checks the crate for usage of weak lang items, returning a vector of all the"] # [doc = " lang items required by this crate, but not defined yet."] pub (crate) fn check_crate (tcx : TyCtxt < '_ > , items : & mut lang_items :: LanguageItems , krate : & ast :: Crate ,) { if items . eh_personality () . is_none () { items . missing . push (LangItem :: EhPersonality) ; } if tcx . sess . target . os == "emscripten" && items . eh_catch_typeinfo () . is_none () && ! tcx . sess . opts . unstable_opts . emscripten_wasm_eh { items . missing . push (LangItem :: EhCatchTypeinfo) ; } visit :: Visitor :: visit_crate (& mut WeakLangItemVisitor { tcx , items } , krate) ; verify (tcx , items) ; }
};
}
