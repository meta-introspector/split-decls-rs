macro_rules! deps {
    () => {
        TraitDecl!();
        Generics!();
        GenericPredicates!();
    };
}

macro_rules! impl_419 {
    () => {
        deps!();
        impl TraitDecl { pub fn generics_of (& self) -> Generics { with (| cx | cx . generics_of (self . def_id . 0)) } pub fn predicates_of (& self) -> GenericPredicates { with (| cx | cx . predicates_of (self . def_id . 0)) } pub fn explicit_predicates_of (& self) -> GenericPredicates { with (| cx | cx . explicit_predicates_of (self . def_id . 0)) } }
    };
}

impl_419!()