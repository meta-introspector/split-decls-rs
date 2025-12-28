macro_rules! deps {
    () => {
        WeakLangItemVisitor!();
        UnknownExternLangItem!();
    };
}

macro_rules! impl_350 {
    () => {
        deps!();
        impl < 'ast > visit :: Visitor < 'ast > for WeakLangItemVisitor < '_ , '_ > { fn visit_foreign_item (& mut self , i : & 'ast ast :: ForeignItem) { if let Some ((lang_item , _)) = lang_items :: extract (& i . attrs) { if let Some (item) = LangItem :: from_name (lang_item) && item . is_weak () { if self . items . get (item) . is_none () { self . items . missing . push (item) ; } } else { self . tcx . dcx () . emit_err (UnknownExternLangItem { span : i . span , lang_item }) ; } } } }
    };
}

impl_350!()