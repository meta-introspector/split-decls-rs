macro_rules! NeedsDropOverflow {
    () => {
        # [derive (Diagnostic)] # [diag (ty_utils_needs_drop_overflow)] pub (crate) struct NeedsDropOverflow < 'tcx > { pub query_ty : Ty < 'tcx > , }
    };
}

NeedsDropOverflow!();