macro_rules! provide {
    () => {
        pub fn provide (providers : & mut Providers) { abi :: provide (providers) ; assoc :: provide (providers) ; common_traits :: provide (providers) ; consts :: provide (providers) ; implied_bounds :: provide (providers) ; layout :: provide (providers) ; needs_drop :: provide (providers) ; opaque_types :: provide (providers) ; representability :: provide (providers) ; ty :: provide (providers) ; instance :: provide (providers) ; structural_match :: provide (providers) ; nested_bodies :: provide (providers) ; }
    };
}

provide!()