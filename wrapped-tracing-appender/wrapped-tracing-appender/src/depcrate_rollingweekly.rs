// Generated macro for weekly (function)
macro_rules! Depcrate_rollingweekly {
() => {
// Module: crate::rolling
// Provides: {"weekly"}
// Dependencies: {}
# [doc = " Creates a weekly-rotating file appender. The logs will rotate every Sunday at midnight UTC."] # [doc = ""] # [doc = " The appender returned by `rolling::weekly` can be used with `non_blocking` to create"] # [doc = " a non-blocking, weekly file appender."] # [doc = ""] # [doc = " A `RollingFileAppender` has a fixed rotation whose frequency is"] # [doc = " defined by [`Rotation`]. The `directory` and `file_name_prefix` arguments"] # [doc = " determine the location and file name's _prefix_ of the log file."] # [doc = " `RollingFileAppender` automatically appends the current date in UTC."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ``` rust"] # [doc = " # #[clippy::allow(needless_doctest_main)]"] # [doc = " fn main () {"] # [doc = " # fn doc() {"] # [doc = "     let appender = tracing_appender::rolling::weekly(\"/some/path\", \"rolling.log\");"] # [doc = "     let (non_blocking_appender, _guard) = tracing_appender::non_blocking(appender);"] # [doc = ""] # [doc = "     let subscriber = tracing_subscriber::fmt().with_writer(non_blocking_appender);"] # [doc = ""] # [doc = "     tracing::subscriber::with_default(subscriber.finish(), || {"] # [doc = "         tracing::event!(tracing::Level::INFO, \"Hello\");"] # [doc = "     });"] # [doc = " # }"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " This will result in a log file located at `/some/path/rolling.log.yyyy-MM-dd`."] pub fn weekly (directory : impl AsRef < Path > , file_name_prefix : impl AsRef < Path > ,) -> RollingFileAppender { RollingFileAppender :: new (Rotation :: WEEKLY , directory , file_name_prefix) }
};
}
