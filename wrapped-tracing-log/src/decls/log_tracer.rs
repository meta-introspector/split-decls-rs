macro_rules! log_tracer {
    () => {
        # [cfg (feature = "log-tracer")] # [cfg_attr (docsrs , doc (cfg (feature = "log-tracer")))] pub mod log_tracer ;
    };
}

log_tracer!();