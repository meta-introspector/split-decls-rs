macro_rules! PanicUnwindWithoutStd {
    () => {
        # [derive (Diagnostic)] # [diag (passes_panic_unwind_without_std)] # [help] # [note] pub (crate) struct PanicUnwindWithoutStd ;
    };
}

PanicUnwindWithoutStd!()