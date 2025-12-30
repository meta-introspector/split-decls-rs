// Generated macro for impl_597 (impl)
macro_rules! Depcrate_weak_lang_itemsimpl_597 {
() => {
// Module: crate::weak_lang_items
// Provides: {"impl_597"}
// Dependencies: {}
impl < 'ast > visit :: Visitor < 'ast > for WeakLangItemVisitor < '_ , '_ > { fn visit_foreign_item (& mut self , i : & 'ast ast :: ForeignItem) { if let Some ((lang_item , _)) = lang_items :: extract (& i . attrs) { if let Some (item) = LangItem :: from_name (lang_item) && item . is_weak () { if self . items . get (item) . is_none () { self . items . missing . push (item) ; } } else { self . tcx . dcx () . emit_err (UnknownExternLangItem { span : i . span , lang_item }) ; } } } }
};
}
