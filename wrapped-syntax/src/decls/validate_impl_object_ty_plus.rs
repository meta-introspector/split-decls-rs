macro_rules! deps {
    () => {
        SyntaxError!();
    };
}

macro_rules! validate_impl_object_ty_plus {
    () => {
        deps!();
        fn validate_impl_object_ty_plus (ty : ast :: ImplTraitType) -> Option < SyntaxError > { let dyn_token = ty . impl_token () ? ; let preceding_token = algo :: skip_trivia_token (dyn_token . prev_token () ? , Direction :: Prev) ? ; let tbl = ty . type_bound_list () ? ; let more_than_one_bound = tbl . bounds () . next_tuple :: < (_ , _) > () . is_some () ; if more_than_one_bound && ! matches ! (preceding_token . kind () , T ! ['('] | T ! [<] | T ! [=]) { Some (SyntaxError :: new ("ambiguous `+` in a type" , ty . syntax () . text_range ())) } else { None } }
    };
}

validate_impl_object_ty_plus!()