macro_rules! LangItems {
    () => {
        pub trait LangItems < 'tcx , T , D > where T : Sized + 'tcx , D : Sized + 'tcx , { fn get_clone_trait_def_id (& self) -> Option < D > ; fn get_debug_trait_def_id (& self) -> Option < D > ; }
    };
}

LangItems!()