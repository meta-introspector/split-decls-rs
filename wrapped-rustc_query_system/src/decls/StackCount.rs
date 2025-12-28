macro_rules! StackCount {
    () => {
        # [derive (Subdiagnostic)] pub (crate) enum StackCount { # [note (query_system_cycle_stack_single)] Single , # [note (query_system_cycle_stack_multiple)] Multiple , }
    };
}

StackCount!()