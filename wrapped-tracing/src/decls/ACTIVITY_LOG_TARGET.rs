macro_rules! ACTIVITY_LOG_TARGET {
    () => {
        # [doc = " `log` target for span activity (enter/exit) records."] # [cfg (feature = "log")] const ACTIVITY_LOG_TARGET : & str = "tracing::span::active" ;
    };
}

ACTIVITY_LOG_TARGET!();