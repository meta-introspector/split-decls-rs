macro_rules! provide {
    () => {
        pub fn provide (p : & mut Providers) { dropck_outlives :: provide (p) ; evaluate_obligation :: provide (p) ; implied_outlives_bounds :: provide (p) ; normalize_projection_ty :: provide (p) ; normalize_erasing_regions :: provide (p) ; type_op :: provide (p) ; p . codegen_select_candidate = codegen :: codegen_select_candidate ; p . coroutine_hidden_types = coroutine_witnesses :: coroutine_hidden_types ; }
    };
}

provide!()