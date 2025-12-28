macro_rules! AllowedPersistOptions {
    () => {
        pub (crate) enum AllowedPersistOptions { AllowedIdent , AllowedValue , Invalid , }
    };
}

AllowedPersistOptions!();