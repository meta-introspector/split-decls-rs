macro_rules! deps {
    () => {
        RustcTyCtxt!();
    };
}

macro_rules! impl_1 {
    () => {
        deps!();
        impl < 'tcx > LangItems < 'tcx , TyCtxt < 'tcx > , DefId > for RustcTyCtxt < 'tcx > { fn get_clone_trait_def_id (& self) -> Option < DefId > { self . 0 . lang_items () . clone_trait () } fn get_debug_trait_def_id (& self) -> Option < DefId > { self . 0 . get_diagnostic_item (sym :: Debug) } }
    };
}

impl_1!()