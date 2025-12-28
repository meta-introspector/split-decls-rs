macro_rules! if_log_enabled {
    () => {
        # [cfg (all (feature = "log" , feature = "log-always"))] # [doc (hidden)] # [macro_export] macro_rules ! if_log_enabled { ($ lvl : expr , $ e : expr ;) => { $ crate :: if_log_enabled ! { $ lvl , $ e } } ; ($ lvl : expr , $ if_log : block) => { $ crate :: if_log_enabled ! { $ lvl , $ if_log else { } } } ; ($ lvl : expr , $ if_log : block else $ else_block : block) => { if $ crate :: level_to_log ! ($ lvl) <= $ crate :: log :: STATIC_MAX_LEVEL { # [allow (unused_braces)] $ if_log } else { $ else_block } } ; }
    };
}

if_log_enabled!();