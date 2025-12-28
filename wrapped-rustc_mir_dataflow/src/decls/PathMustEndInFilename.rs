macro_rules! PathMustEndInFilename {
    () => {
        # [derive (Diagnostic)] # [diag (mir_dataflow_path_must_end_in_filename)] pub (crate) struct PathMustEndInFilename { # [primary_span] pub span : Span , }
    };
}

PathMustEndInFilename!()