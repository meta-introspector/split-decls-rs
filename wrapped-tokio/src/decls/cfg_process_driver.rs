macro_rules! cfg_process_driver {
    () => {
        macro_rules ! cfg_process_driver { ($ ($ item : item) *) => { # [cfg (unix)] # [cfg (not (loom))] cfg_process ! { $ ($ item) * } } }
    };
}

cfg_process_driver!()