macro_rules! TypeNotStructural {
    () => {
        # [derive (Diagnostic)] # [diag (mir_build_type_not_structural)] pub (crate) struct TypeNotStructural < 'tcx > { # [primary_span] # [label] pub (crate) span : Span , # [label (mir_build_type_not_structural_def)] pub (crate) ty_def_span : Span , pub (crate) ty : Ty < 'tcx > , # [note (mir_build_type_not_structural_tip)] pub (crate) manual_partialeq_impl_span : Option < Span > , # [note (mir_build_type_not_structural_more_info)] pub (crate) manual_partialeq_impl_note : bool , }
    };
}

TypeNotStructural!()