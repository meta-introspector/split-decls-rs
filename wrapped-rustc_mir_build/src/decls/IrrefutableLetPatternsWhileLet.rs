macro_rules! IrrefutableLetPatternsWhileLet {
    () => {
        # [derive (LintDiagnostic)] # [diag (mir_build_irrefutable_let_patterns_while_let)] # [note] # [help] pub (crate) struct IrrefutableLetPatternsWhileLet { pub (crate) count : usize , }
    };
}

IrrefutableLetPatternsWhileLet!();