macro_rules! ObjectLifetimeErr {
    () => {
        # [derive (Diagnostic)] # [diag (passes_object_lifetime_err)] pub (crate) struct ObjectLifetimeErr { # [primary_span] pub span : Span , pub repr : String , }
    };
}

ObjectLifetimeErr!();