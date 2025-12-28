macro_rules! __tracing_log {
    () => {
        # [cfg (feature = "log")] # [doc (hidden)] # [macro_export] macro_rules ! __tracing_log { ($ level : expr , $ callsite : expr , $ value_set : expr) => { $ crate :: if_log_enabled ! { $ level , { use $ crate :: log ; let level = $ crate :: level_to_log ! ($ level) ; if level <= log :: max_level () { let meta = $ callsite . metadata () ; let log_meta = log :: Metadata :: builder () . level (level) . target (meta . target ()) . build () ; let logger = log :: logger () ; if logger . enabled (& log_meta) { $ crate :: __macro_support :: __tracing_log (meta , logger , log_meta , $ value_set) } } } } } ; }
    };
}

__tracing_log!()