macro_rules! deps {
    () => {
        FutureIncompatibleInfo!();
        FutureIncompatibilityReason!();
    };
}

macro_rules! impl_151 {
    () => {
        deps!();
        impl FutureIncompatibleInfo { pub const fn default_fields_for_macro () -> Self { FutureIncompatibleInfo { reference : "" , reason : FutureIncompatibilityReason :: FutureReleaseError , explain_reason : true , report_in_deps : false , } } }
    };
}

impl_151!()