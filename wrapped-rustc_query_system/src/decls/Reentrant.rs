macro_rules! Reentrant {
    () => {
        # [derive (Diagnostic)] # [diag (query_system_reentrant)] pub (crate) struct Reentrant ;
    };
}

Reentrant!()