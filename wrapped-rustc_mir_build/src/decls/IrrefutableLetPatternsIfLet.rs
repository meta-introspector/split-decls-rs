macro_rules! IrrefutableLetPatternsIfLet {
    () => {
        # [derive (LintDiagnostic)] # [diag (mir_build_irrefutable_let_patterns_if_let)] # [note] # [help] pub (crate) struct IrrefutableLetPatternsIfLet { pub (crate) count : usize , }
    };
}

IrrefutableLetPatternsIfLet!()