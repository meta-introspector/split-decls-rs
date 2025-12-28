macro_rules! deps {
    () => {
        TraitDecl!();
    };
}

macro_rules! impl_369 {
    () => {
        deps!();
        impl TraitDef { pub fn declaration (trait_def : & TraitDef) -> TraitDecl { with (| cx | cx . trait_decl (trait_def)) } }
    };
}

impl_369!()