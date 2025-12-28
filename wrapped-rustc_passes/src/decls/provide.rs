macro_rules! provide {
    () => {
        pub fn provide (providers : & mut Providers) { check_attr :: provide (providers) ; dead :: provide (providers) ; debugger_visualizer :: provide (providers) ; diagnostic_items :: provide (providers) ; entry :: provide (providers) ; lang_items :: provide (providers) ; lib_features :: provide (providers) ; liveness :: provide (providers) ; reachable :: provide (providers) ; stability :: provide (providers) ; upvars :: provide (providers) ; check_export :: provide (providers) ; }
    };
}

provide!();