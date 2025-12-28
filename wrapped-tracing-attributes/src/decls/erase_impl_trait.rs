macro_rules! deps {
    () => {
        ImplTraitEraser!();
    };
}

macro_rules! erase_impl_trait {
    () => {
        deps!();
        fn erase_impl_trait (ty : & Type) -> Type { let mut ty = ty . clone () ; ImplTraitEraser . visit_type_mut (& mut ty) ; ty }
    };
}

erase_impl_trait!()