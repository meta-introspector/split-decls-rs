macro_rules! IrrefutableLetPatternsLetElse {
    () => {
        # [derive (LintDiagnostic)] # [diag (mir_build_irrefutable_let_patterns_let_else)] # [note] # [help] pub (crate) struct IrrefutableLetPatternsLetElse { pub (crate) count : usize , }
    };
}

IrrefutableLetPatternsLetElse!();