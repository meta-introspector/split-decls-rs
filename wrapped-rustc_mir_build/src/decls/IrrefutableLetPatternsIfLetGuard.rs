macro_rules! IrrefutableLetPatternsIfLetGuard {
    () => {
        # [derive (LintDiagnostic)] # [diag (mir_build_irrefutable_let_patterns_if_let_guard)] # [note] # [help] pub (crate) struct IrrefutableLetPatternsIfLetGuard { pub (crate) count : usize , }
    };
}

IrrefutableLetPatternsIfLetGuard!();