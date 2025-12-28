macro_rules! UnreachableDueToUninhabited {
    () => {
        # [derive (LintDiagnostic)] # [diag (passes_unreachable_due_to_uninhabited)] pub (crate) struct UnreachableDueToUninhabited < 'desc , 'tcx > { pub descr : & 'desc str , # [label] pub expr : Span , # [label (passes_label_orig)] # [note] pub orig : Span , pub ty : Ty < 'tcx > , }
    };
}

UnreachableDueToUninhabited!();