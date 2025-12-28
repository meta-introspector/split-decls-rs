macro_rules! CycleStack {
    () => {
        # [derive (Subdiagnostic)] # [note (query_system_cycle_stack_middle)] pub (crate) struct CycleStack { # [primary_span] pub span : Span , pub desc : String , }
    };
}

CycleStack!()