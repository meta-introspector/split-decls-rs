macro_rules! deps {
    () => {
        ReachEverythingInTheInterfaceVisitor!();
    };
}

macro_rules! impl_27 {
    () => {
        deps!();
        impl ReachEverythingInTheInterfaceVisitor < '_ , '_ > { fn generics (& mut self) -> & mut Self { for param in & self . ev . tcx . generics_of (self . item_def_id) . own_params { if let GenericParamDefKind :: Const { .. } = param . kind { self . visit (self . ev . tcx . type_of (param . def_id) . instantiate_identity ()) ; } if let Some (default) = param . default_value (self . ev . tcx) { self . visit (default . instantiate_identity ()) ; } } self } fn predicates (& mut self) -> & mut Self { self . visit_predicates (self . ev . tcx . predicates_of (self . item_def_id)) ; self } fn ty (& mut self) -> & mut Self { self . visit (self . ev . tcx . type_of (self . item_def_id) . instantiate_identity ()) ; self } fn trait_ref (& mut self) -> & mut Self { if let Some (trait_ref) = self . ev . tcx . impl_trait_ref (self . item_def_id) { self . visit_trait (trait_ref . instantiate_identity ()) ; } self } }
    };
}

impl_27!()