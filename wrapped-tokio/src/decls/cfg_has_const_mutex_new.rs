macro_rules! cfg_has_const_mutex_new {
    () => {
        macro_rules ! cfg_has_const_mutex_new { ($ ($ item : item) *) => { $ (# [cfg (not (all (loom , test)))] $ item) * } }
    };
}

cfg_has_const_mutex_new!();