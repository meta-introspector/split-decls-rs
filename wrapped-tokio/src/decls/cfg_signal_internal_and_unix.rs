macro_rules! cfg_signal_internal_and_unix {
    () => {
        macro_rules ! cfg_signal_internal_and_unix { ($ ($ item : item) *) => { # [cfg (unix)] cfg_signal_internal ! { $ ($ item) * } } }
    };
}

cfg_signal_internal_and_unix!()