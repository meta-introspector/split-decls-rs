macro_rules! provide {
    () => {
        pub fn provide (providers : & mut Providers) { partitioning :: provide (providers) ; mono_checks :: provide (providers) ; }
    };
}

provide!()