macro_rules! deps {
    () => {
        SyntaxError!();
    };
}

macro_rules! validate_trait_object_ref_ty {
    () => {
        deps!();
        fn validate_trait_object_ref_ty (ty : ast :: RefType , errors : & mut Vec < SyntaxError >) { match ty . ty () { Some (ast :: Type :: DynTraitType (ty)) => { if let Some (err) = validate_trait_object_ty_plus (ty) { errors . push (err) ; } } Some (ast :: Type :: ImplTraitType (ty)) => { if let Some (err) = validate_impl_object_ty_plus (ty) { errors . push (err) ; } } _ => () , } }
    };
}

validate_trait_object_ref_ty!();