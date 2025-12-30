// Generated macro for daily (function)
macro_rules! Depcrate_rollingdaily {
() => {
// Module: crate::rolling
// Provides: {"daily"}
// Dependencies: {}
# [doc = " Creates a daily-rotating file appender."] # [doc = ""] # [doc = " The appender returned by `rolling::daily` can be used with `non_blocking` to create"] # [doc = " a non-blocking, daily file appender."] # [doc = ""] # [doc = " A `RollingFileAppender` has a fixed rotation whose frequency is"] # [doc = " defined by [`Rotation`]. The `directory` and `file_name_prefix`"] # [doc = " arguments determine the location and file name's _prefix_ of the log file."] # [doc = " `RollingFileAppender` automatically appends the current date in UTC."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ``` rust"] # [doc = " # #[clippy::allow(needless_doctest_main)]"] # [doc = " fn main () {"] # [doc = " # fn doc() {"] # [doc = "     let appender = tracing_appender::rolling::daily(\"/some/path\", \"rolling.log\");"] # [doc = "     let (non_blocking_appender, _guard) = tracing_appender::non_blocking(appender);"] # [doc = ""] # [doc = "     let subscriber = tracing_subscriber::fmt().with_writer(non_blocking_appender);"] # [doc = ""] # [doc = "     tracing::subscriber::with_default(subscriber.finish(), || {"] # [doc = "         tracing::event!(tracing::Level::INFO, \"Hello\");"] # [doc = "     });"] # [doc = " # }"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " This will result in a log file located at `/some/path/rolling.log.yyyy-MM-dd`."] pub fn daily (directory : impl AsRef < Path > , file_name_prefix : impl AsRef < Path > ,) -> RollingFileAppender { RollingFileAppender :: new (Rotation :: DAILY , directory , file_name_prefix) }
};
}
