macro_rules! deps {
    () => {
        DiagnosticDeriveError!();
    };
}

macro_rules! throw_invalid_attr {
    () => {
        deps!();
        # [doc = " Emit an error diagnostic for an invalid attribute (optionally performing additional decoration"] # [doc = " using the `FnOnce` passed in `diag`) and return `Err(ErrorHandled)`."] # [doc = ""] # [doc = " For methods that return a `Result<_, DiagnosticDeriveError>`:"] macro_rules ! throw_invalid_attr { ($ attr : expr) => { { throw_invalid_attr ! ($ attr , | diag | diag) } } ; ($ attr : expr , $ f : expr) => { { let diag = crate :: diagnostics :: error :: invalid_attr ($ attr) ; return Err (crate :: diagnostics :: error :: _throw_err (diag , $ f)) ; } } ; }
    };
}

throw_invalid_attr!();