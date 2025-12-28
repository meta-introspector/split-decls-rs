macro_rules! cfg_not_has_const_mutex_new {
    () => {
        macro_rules ! cfg_not_has_const_mutex_new { ($ ($ item : item) *) => { $ (# [cfg (all (loom , test))] $ item) * } }
    };
}

cfg_not_has_const_mutex_new!();