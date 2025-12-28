macro_rules! slog_support {
    () => {
        # [cfg (feature = "slog")] pub (crate) mod slog_support ;
    };
}

slog_support!();