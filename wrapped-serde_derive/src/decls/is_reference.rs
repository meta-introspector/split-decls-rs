macro_rules! is_reference {
    () => {
        fn is_reference (ty : & syn :: Type , elem : fn (& syn :: Type) -> bool) -> bool { match ungroup (ty) { syn :: Type :: Reference (ty) => ty . mutability . is_none () && elem (& ty . elem) , _ => false , } }
    };
}

is_reference!();