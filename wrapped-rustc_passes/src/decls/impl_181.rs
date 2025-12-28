macro_rules! deps {
    () => {
        NoMainErr!();
    };
}

macro_rules! impl_181 {
    () => {
        deps!();
        impl < 'a , G : EmissionGuarantee > Diagnostic < 'a , G > for NoMainErr { # [track_caller] fn into_diag (self , dcx : DiagCtxtHandle < 'a > , level : Level) -> Diag < 'a , G > { let mut diag = Diag :: new (dcx , level , fluent :: passes_no_main_function) ; diag . span (DUMMY_SP) ; diag . code (E0601) ; diag . arg ("crate_name" , self . crate_name) ; diag . arg ("filename" , self . filename) ; diag . arg ("has_filename" , self . has_filename) ; let note = if ! self . non_main_fns . is_empty () { for & span in & self . non_main_fns { diag . span_note (span , fluent :: passes_here_is_main) ; } diag . note (fluent :: passes_one_or_more_possible_main) ; diag . help (fluent :: passes_consider_moving_main) ; fluent :: passes_main_must_be_defined_at_crate } else if self . has_filename { fluent :: passes_consider_adding_main_to_file } else { fluent :: passes_consider_adding_main_at_crate } ; if self . file_empty { diag . note (note) ; } else { diag . span (self . sp . shrink_to_hi ()) ; diag . span_label (self . sp . shrink_to_hi () , note) ; } if let Some (main_def) = self . main_def_opt && main_def . opt_fn_def_id () . is_none () { diag . span_label (main_def . span , fluent :: passes_non_function_main) ; } if self . add_teach_note { diag . note (fluent :: passes_teach_note) ; } diag } }
    };
}

impl_181!();