macro_rules! deps {
    () => {
        SyntaxError!();
    };
}

macro_rules! validate_trait_object_ty {
    () => {
        deps!();
        fn validate_trait_object_ty (ty : ast :: DynTraitType) -> Option < SyntaxError > { let tbl = ty . type_bound_list () ? ; let no_bounds = tbl . bounds () . filter_map (| it | it . ty ()) . next () . is_none () ; match no_bounds { true => Some (SyntaxError :: new ("At least one trait is required for an object type" , ty . syntax () . text_range () ,)) , false => None , } }
    };
}

validate_trait_object_ty!();