macro_rules! deps {
    () => {
        CycleStack!();
        Alias!();
        CycleUsage!();
        StackCount!();
    };
}

macro_rules! Cycle {
    () => {
        deps!();
        # [derive (Diagnostic)] # [diag (query_system_cycle , code = E0391)] pub (crate) struct Cycle { # [primary_span] pub span : Span , pub stack_bottom : String , # [subdiagnostic] pub cycle_stack : Vec < CycleStack > , # [subdiagnostic] pub stack_count : StackCount , # [subdiagnostic] pub alias : Option < Alias > , # [subdiagnostic] pub cycle_usage : Option < CycleUsage > , # [note] pub note_span : () , }
    };
}

Cycle!();