macro_rules! IneffectiveUnstableImpl {
    () => {
        # [derive (LintDiagnostic)] # [diag (passes_ineffective_unstable_impl)] # [note] pub (crate) struct IneffectiveUnstableImpl ;
    };
}

IneffectiveUnstableImpl!()