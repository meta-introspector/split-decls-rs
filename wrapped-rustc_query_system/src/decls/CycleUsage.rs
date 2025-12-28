macro_rules! CycleUsage {
    () => {
        # [derive (Subdiagnostic)] # [note (query_system_cycle_usage)] pub (crate) struct CycleUsage { # [primary_span] pub span : Span , pub usage : String , }
    };
}

CycleUsage!();