macro_rules! macro_0 {
    () => {
        # [cfg (all (thiserror_nightly_testing , not (error_generic_member_access)))] compile_error ! ("Build script probe failed to compile.") ;
    };
}

macro_0!();