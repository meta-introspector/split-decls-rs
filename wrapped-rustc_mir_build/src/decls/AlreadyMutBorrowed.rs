macro_rules! deps {
    () => {
        Conflict!();
    };
}

macro_rules! AlreadyMutBorrowed {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (mir_build_already_mut_borrowed)] pub (crate) struct AlreadyMutBorrowed { # [primary_span] pub (crate) span : Span , # [subdiagnostic] pub (crate) occurrences : Vec < Conflict > , }
    };
}

AlreadyMutBorrowed!();