// Generated macro for verify (function)
macro_rules! Depcrate_weak_lang_itemsverify {
() => {
// Module: crate::weak_lang_items
// Provides: {"verify"}
// Dependencies: {}
fn verify (tcx : TyCtxt < '_ > , items : & lang_items :: LanguageItems) { let needs_check = tcx . crate_types () . iter () . any (| kind | match * kind { CrateType :: Dylib | CrateType :: ProcMacro | CrateType :: Cdylib | CrateType :: Executable | CrateType :: Staticlib | CrateType :: Sdylib => true , CrateType :: Rlib => false , }) ; if ! needs_check { return ; } let mut missing = FxHashSet :: default () ; for & cnum in tcx . crates (()) . iter () { for & item in tcx . missing_lang_items (cnum) . iter () { missing . insert (item) ; } } for & item in WEAK_LANG_ITEMS . iter () { if missing . contains (& item) && required (tcx , item) && items . get (item) . is_none () { if item == LangItem :: PanicImpl { tcx . dcx () . emit_err (MissingPanicHandler) ; } else if item == LangItem :: EhPersonality { tcx . dcx () . emit_err (PanicUnwindWithoutStd) ; } else { tcx . dcx () . emit_err (MissingLangItem { name : item . name () }) ; } } } }
};
}
