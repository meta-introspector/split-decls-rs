macro_rules! DuplicateValuesFor {
    () => {
        # [derive (Diagnostic)] # [diag (mir_dataflow_duplicate_values_for)] pub (crate) struct DuplicateValuesFor { # [primary_span] pub span : Span , pub name : Symbol , }
    };
}

DuplicateValuesFor!()