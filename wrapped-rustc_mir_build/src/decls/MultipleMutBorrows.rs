macro_rules! deps {
    () => {
        Conflict!();
    };
}

macro_rules! MultipleMutBorrows {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (mir_build_multiple_mut_borrows)] pub (crate) struct MultipleMutBorrows { # [primary_span] pub (crate) span : Span , # [subdiagnostic] pub (crate) occurrences : Vec < Conflict > , }
    };
}

MultipleMutBorrows!()