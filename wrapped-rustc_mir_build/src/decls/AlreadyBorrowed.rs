macro_rules! deps {
    () => {
        Conflict!();
    };
}

macro_rules! AlreadyBorrowed {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (mir_build_already_borrowed)] pub (crate) struct AlreadyBorrowed { # [primary_span] pub (crate) span : Span , # [subdiagnostic] pub (crate) occurrences : Vec < Conflict > , }
    };
}

AlreadyBorrowed!()