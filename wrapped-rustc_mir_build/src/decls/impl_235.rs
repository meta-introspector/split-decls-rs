macro_rules! deps {
    () => {
        NonExhaustivePatternsTypeNotEmpty!();
    };
}

macro_rules! impl_235 {
    () => {
        deps!();
        impl < 'a , G : EmissionGuarantee > Diagnostic < 'a , G > for NonExhaustivePatternsTypeNotEmpty < '_ , '_ , '_ > { fn into_diag (self , dcx : DiagCtxtHandle < 'a > , level : Level) -> Diag < 'a , G > { let mut diag = Diag :: new (dcx , level , fluent :: mir_build_non_exhaustive_patterns_type_not_empty) ; diag . span (self . scrut_span) ; diag . code (E0004) ; let peeled_ty = self . ty . peel_refs () ; diag . arg ("ty" , self . ty) ; diag . arg ("peeled_ty" , peeled_ty) ; if let ty :: Adt (def , _) = peeled_ty . kind () { let def_span = self . cx . tcx . hir_get_if_local (def . did ()) . and_then (| node | node . ident ()) . map (| ident | ident . span) . unwrap_or_else (| | self . cx . tcx . def_span (def . did ())) ; let mut span : MultiSpan = def_span . into () ; span . push_span_label (def_span , "") ; diag . span_note (span , fluent :: mir_build_def_note) ; } let is_non_exhaustive = matches ! (self . ty . kind () , ty :: Adt (def , _) if def . variant_list_has_applicable_non_exhaustive ()) ; if is_non_exhaustive { diag . note (fluent :: mir_build_non_exhaustive_type_note) ; } else { diag . note (fluent :: mir_build_type_note) ; } if let ty :: Ref (_ , sub_ty , _) = self . ty . kind () { if ! sub_ty . is_inhabited_from (self . cx . tcx , self . cx . module , self . cx . typing_env) { diag . note (fluent :: mir_build_reference_note) ; } } let sm = self . cx . tcx . sess . source_map () ; if let Some (braces_span) = self . braces_span { let (indentation , more) = if let Some (snippet) = sm . indentation_before (self . scrut_span) { (format ! ("\n{snippet}") , "    ") } else { (" " . to_string () , "") } ; diag . span_suggestion_verbose (braces_span , fluent :: mir_build_suggestion , format ! (" {{{indentation}{more}_ => todo!(),{indentation}}}") , Applicability :: HasPlaceholders ,) ; } else { diag . help (fluent :: mir_build_help) ; } diag } }
    };
}

impl_235!();