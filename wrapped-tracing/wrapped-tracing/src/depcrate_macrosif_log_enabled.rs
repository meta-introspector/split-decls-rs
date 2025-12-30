// Generated macro for if_log_enabled (macro)
macro_rules! Depcrate_macrosif_log_enabled {
() => {
// Module: crate::macros
// Provides: {"if_log_enabled"}
// Dependencies: {}
# [cfg (all (feature = "log" , feature = "log-always"))] # [doc (hidden)] # [macro_export] macro_rules ! if_log_enabled { ($ lvl : expr , $ e : expr ;) => { $ crate :: if_log_enabled ! { $ lvl , $ e } } ; ($ lvl : expr , $ if_log : block) => { $ crate :: if_log_enabled ! { $ lvl , $ if_log else { } } } ; ($ lvl : expr , $ if_log : block else $ else_block : block) => { if $ crate :: level_to_log ! ($ lvl) <= $ crate :: log :: STATIC_MAX_LEVEL { # [allow (unused_braces)] $ if_log } else { $ else_block } } ; }
};
}
