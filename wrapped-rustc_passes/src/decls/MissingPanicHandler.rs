macro_rules! MissingPanicHandler {
    () => {
        # [derive (Diagnostic)] # [diag (passes_missing_panic_handler)] pub (crate) struct MissingPanicHandler ;
    };
}

MissingPanicHandler!()