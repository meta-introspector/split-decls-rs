macro_rules! LeadingIrrefutableLetPatterns {
    () => {
        # [derive (LintDiagnostic)] # [diag (mir_build_leading_irrefutable_let_patterns)] # [note] # [help] pub (crate) struct LeadingIrrefutableLetPatterns { pub (crate) count : usize , }
    };
}

LeadingIrrefutableLetPatterns!();