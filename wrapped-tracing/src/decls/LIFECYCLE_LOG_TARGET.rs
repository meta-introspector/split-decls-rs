macro_rules! LIFECYCLE_LOG_TARGET {
    () => {
        # [doc = " `log` target for all span lifecycle (creation/enter/exit/close) records."] # [cfg (feature = "log")] const LIFECYCLE_LOG_TARGET : & str = "tracing::span" ;
    };
}

LIFECYCLE_LOG_TARGET!();