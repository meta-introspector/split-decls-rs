macro_rules! deps {
    () => {
        LocalLabel!();
    };
}

macro_rules! TailExprDropOrderLint {
    () => {
        deps!();
        # [derive (LintDiagnostic)] # [diag (mir_transform_tail_expr_drop_order)] struct TailExprDropOrderLint < 'a > { # [subdiagnostic] local_labels : Vec < LocalLabel < 'a > > , # [label (mir_transform_drop_location)] drop_span : Option < Span > , # [note (mir_transform_note_epilogue)] _epilogue : () , }
    };
}

TailExprDropOrderLint!()