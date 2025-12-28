macro_rules! deps {
    () => {
        ProcMacroKind!();
    };
}

macro_rules! ProcMacroBadSig {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (passes_proc_macro_bad_sig)] pub (crate) struct ProcMacroBadSig { # [primary_span] pub span : Span , pub kind : ProcMacroKind , }
    };
}

ProcMacroBadSig!()