// Generated macro for validate_trait_object_ty_plus (function)
macro_rules! Depcrate_validationvalidate_trait_object_ty_plus {
() => {
// Module: crate::validation
// Provides: {"validate_trait_object_ty_plus"}
// Dependencies: {}
fn validate_trait_object_ty_plus (ty : ast :: DynTraitType) -> Option < SyntaxError > { let dyn_token = ty . dyn_token () ? ; let preceding_token = algo :: skip_trivia_token (dyn_token . prev_token () ? , Direction :: Prev) ? ; let tbl = ty . type_bound_list () ? ; let more_than_one_bound = tbl . bounds () . next_tuple :: < (_ , _) > () . is_some () ; if more_than_one_bound && ! matches ! (preceding_token . kind () , T ! ['('] | T ! [<] | T ! [=]) { Some (SyntaxError :: new ("ambiguous `+` in a type" , ty . syntax () . text_range ())) } else { None } }
};
}
