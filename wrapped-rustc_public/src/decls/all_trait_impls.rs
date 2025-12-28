macro_rules! deps {
    () => {
        ImplTraitDecls!();
    };
}

macro_rules! all_trait_impls {
    () => {
        deps!();
        pub fn all_trait_impls () -> ImplTraitDecls { with (| cx | cx . all_trait_impls ()) }
    };
}

all_trait_impls!()