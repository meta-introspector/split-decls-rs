macro_rules! TrailingIrrefutableLetPatterns {
    () => {
        # [derive (LintDiagnostic)] # [diag (mir_build_trailing_irrefutable_let_patterns)] # [note] # [help] pub (crate) struct TrailingIrrefutableLetPatterns { pub (crate) count : usize , }
    };
}

TrailingIrrefutableLetPatterns!()