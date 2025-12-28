macro_rules! InterpretedAsConst {
    () => {
        # [derive (Subdiagnostic)] # [label (mir_build_confused)] pub (crate) struct InterpretedAsConst { # [primary_span] pub (crate) span : Span , pub (crate) variable : String , }
    };
}

InterpretedAsConst!();