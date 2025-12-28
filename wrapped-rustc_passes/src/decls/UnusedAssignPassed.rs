macro_rules! UnusedAssignPassed {
    () => {
        # [derive (LintDiagnostic)] # [diag (passes_unused_assign_passed)] # [help] pub (crate) struct UnusedAssignPassed { pub name : String , }
    };
}

UnusedAssignPassed!()