macro_rules! cfg_not_process_driver {
    () => {
        macro_rules ! cfg_not_process_driver { ($ ($ item : item) *) => { $ (# [cfg (not (all (unix , not (loom) , feature = "process")))] $ item) * } }
    };
}

cfg_not_process_driver!();