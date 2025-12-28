macro_rules! macro_259 {
    () => {
        cfg_signal_internal ! { # [cfg (not (feature = "signal"))] # [allow (dead_code)] # [allow (unreachable_pub)] pub (crate) mod signal ; }
    };
}

macro_259!()