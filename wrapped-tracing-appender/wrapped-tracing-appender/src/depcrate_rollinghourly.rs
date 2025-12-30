// Generated macro for hourly (function)
macro_rules! Depcrate_rollinghourly {
() => {
// Module: crate::rolling
// Provides: {"hourly"}
// Dependencies: {}
# [doc = " Creates an hourly-rotating file appender."] # [doc = ""] # [doc = " The appender returned by `rolling::hourly` can be used with `non_blocking` to create"] # [doc = " a non-blocking, hourly file appender."] # [doc = ""] # [doc = " The directory of the log file is specified with the `directory` argument."] # [doc = " `file_name_prefix` specifies the _prefix_ of the log file. `RollingFileAppender`"] # [doc = " adds the current date and hour to the log file in UTC."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ``` rust"] # [doc = " # #[clippy::allow(needless_doctest_main)]"] # [doc = " fn main () {"] # [doc = " # fn doc() {"] # [doc = "     let appender = tracing_appender::rolling::hourly(\"/some/path\", \"rolling.log\");"] # [doc = "     let (non_blocking_appender, _guard) = tracing_appender::non_blocking(appender);"] # [doc = ""] # [doc = "     let subscriber = tracing_subscriber::fmt().with_writer(non_blocking_appender);"] # [doc = ""] # [doc = "     tracing::subscriber::with_default(subscriber.finish(), || {"] # [doc = "         tracing::event!(tracing::Level::INFO, \"Hello\");"] # [doc = "     });"] # [doc = " # }"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " This will result in a log file located at `/some/path/rolling.log.yyyy-MM-dd-HH`."] pub fn hourly (directory : impl AsRef < Path > , file_name_prefix : impl AsRef < Path > ,) -> RollingFileAppender { RollingFileAppender :: new (Rotation :: HOURLY , directory , file_name_prefix) }
};
}
