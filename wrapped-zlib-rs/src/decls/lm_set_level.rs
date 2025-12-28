macro_rules! deps {
    () => {
        State!();
    };
}

macro_rules! lm_set_level {
    () => {
        deps!();
        fn lm_set_level (state : & mut State , level : i8) { state . max_lazy_match = CONFIGURATION_TABLE [level as usize] . max_lazy ; state . good_match = CONFIGURATION_TABLE [level as usize] . good_length ; state . nice_match = CONFIGURATION_TABLE [level as usize] . nice_length ; state . max_chain_length = CONFIGURATION_TABLE [level as usize] . max_chain ; state . hash_calc_variant = HashCalcVariant :: for_max_chain_length (state . max_chain_length) ; state . level = level ; }
    };
}

lm_set_level!()