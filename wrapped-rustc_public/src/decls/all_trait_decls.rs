macro_rules! deps {
    () => {
        TraitDecls!();
    };
}

macro_rules! all_trait_decls {
    () => {
        deps!();
        pub fn all_trait_decls () -> TraitDecls { with (| cx | cx . all_trait_decls ()) }
    };
}

all_trait_decls!();