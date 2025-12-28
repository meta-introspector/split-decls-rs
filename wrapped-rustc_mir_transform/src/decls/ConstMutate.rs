macro_rules! ConstMutate {
    () => {
        # [derive (LintDiagnostic)] pub (crate) enum ConstMutate { # [diag (mir_transform_const_modify)] # [note] Modify { # [note (mir_transform_const_defined_here)] konst : Span , } , # [diag (mir_transform_const_mut_borrow)] # [note] # [note (mir_transform_note2)] MutBorrow { # [note (mir_transform_note3)] method_call : Option < Span > , # [note (mir_transform_const_defined_here)] konst : Span , } , }
    };
}

ConstMutate!()