// Generated macro for setup_tracked_assoc_fn_body (macro)
macro_rules! Depcrate_setup_tracked_assoc_fn_bodysetup_tracked_assoc_fn_body {
() => {
// Module: crate::setup_tracked_assoc_fn_body
// Provides: {"setup_tracked_assoc_fn_body"}
// Dependencies: {}
# [macro_export] macro_rules ! setup_tracked_assoc_fn_body { (salsa_tracked_attr : # [$ salsa_tracked_attr : meta] , self_ty : $ self_ty : ty , db_lt : $ ($ db_lt : lifetime) ?, db : $ db : ident , db_ty : ($ ($ db_ty : tt) *) , input_ids : [$ ($ input_id : ident) ,*] , input_tys : [$ ($ input_ty : ty) ,*] , output_ty : $ output_ty : ty , inner_fn_name : $ inner_fn_name : ident , inner_fn : $ inner_fn : item , unused_names : [$ InnerTrait : ident ,]) => { { trait $ InnerTrait <$ ($ db_lt) ?> { fn $ inner_fn_name (db : $ ($ db_ty) *, $ ($ input_id : $ input_ty) ,*) -> $ output_ty ; } impl <$ ($ db_lt) ?> $ InnerTrait <$ ($ db_lt) ?> for $ self_ty { $ inner_fn } # [$ salsa_tracked_attr] fn $ inner_fn_name <$ ($ db_lt) ?> (db : $ ($ db_ty) *, $ ($ input_id : $ input_ty) ,*) -> $ output_ty { <$ self_ty as $ InnerTrait >::$ inner_fn_name (db , $ ($ input_id) ,*) } $ inner_fn_name ($ db , $ ($ input_id) ,*) } } ; }
};
}
