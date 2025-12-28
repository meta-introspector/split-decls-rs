macro_rules! Inform {
    () => {
        # [derive (Subdiagnostic)] # [note (mir_build_inform_irrefutable)] # [note (mir_build_more_information)] pub (crate) struct Inform ;
    };
}

Inform!();