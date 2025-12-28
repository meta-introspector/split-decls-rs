macro_rules! deps {
    () => {
        DiagnosticDeriveError!();
    };
}

macro_rules! throw_span_err {
    () => {
        deps!();
        # [doc = " Emit a diagnostic on span `$span` with msg `$msg` (optionally performing additional decoration"] # [doc = " using the `FnOnce` passed in `diag`) and return `Err(ErrorHandled)`."] # [doc = ""] # [doc = " For methods that return a `Result<_, DiagnosticDeriveError>`:"] macro_rules ! throw_span_err { ($ span : expr , $ msg : expr) => { { throw_span_err ! ($ span , $ msg , | diag | diag) } } ; ($ span : expr , $ msg : expr , $ f : expr) => { { let diag = span_err ($ span , $ msg) ; return Err (crate :: diagnostics :: error :: _throw_err (diag , $ f)) ; } } ; }
    };
}

throw_span_err!();