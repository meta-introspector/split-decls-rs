macro_rules! deps {
    () => {
        SyntaxError!();
    };
}

macro_rules! validate_trait_object_fn_ptr_ret_ty {
    () => {
        deps!();
        fn validate_trait_object_fn_ptr_ret_ty (ty : ast :: FnPtrType , errors : & mut Vec < SyntaxError >) { match ty . ret_type () . and_then (| ty | ty . ty ()) { Some (ast :: Type :: DynTraitType (ty)) => { if let Some (err) = validate_trait_object_ty_plus (ty) { errors . push (err) ; } } Some (ast :: Type :: ImplTraitType (ty)) => { if let Some (err) = validate_impl_object_ty_plus (ty) { errors . push (err) ; } } _ => () , } }
    };
}

validate_trait_object_fn_ptr_ret_ty!();