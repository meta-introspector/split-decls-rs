macro_rules! deps {
    () => {
        Conflict!();
    };
}

macro_rules! MovedWhileBorrowed {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (mir_build_moved_while_borrowed)] pub (crate) struct MovedWhileBorrowed { # [primary_span] pub (crate) span : Span , # [subdiagnostic] pub (crate) occurrences : Vec < Conflict > , }
    };
}

MovedWhileBorrowed!();