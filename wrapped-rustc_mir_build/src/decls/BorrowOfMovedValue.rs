macro_rules! BorrowOfMovedValue {
    () => {
        # [derive (Diagnostic)] # [diag (mir_build_borrow_of_moved_value)] pub (crate) struct BorrowOfMovedValue < 'tcx > { # [primary_span] # [label] # [label (mir_build_occurs_because_label)] pub (crate) binding_span : Span , # [label (mir_build_value_borrowed_label)] pub (crate) conflicts_ref : Vec < Span > , pub (crate) name : Ident , pub (crate) ty : Ty < 'tcx > , # [suggestion (code = "ref " , applicability = "machine-applicable")] pub (crate) suggest_borrowing : Option < Span > , }
    };
}

BorrowOfMovedValue!();