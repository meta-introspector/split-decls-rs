// Generated macro for never (function)
macro_rules! Depcrate_rollingnever {
() => {
// Module: crate::rolling
// Provides: {"never"}
// Dependencies: {}
# [doc = " Creates a non-rolling file appender."] # [doc = ""] # [doc = " The appender returned by `rolling::never` can be used with `non_blocking` to create"] # [doc = " a non-blocking, non-rotating appender."] # [doc = ""] # [doc = " The location of the log file will be specified the `directory` passed in."] # [doc = " `file_name` specifies the complete name of the log file (no date or time is appended)."] # [doc = ""] # [doc = " # Examples"] # [doc = ""] # [doc = " ``` rust"] # [doc = " # #[clippy::allow(needless_doctest_main)]"] # [doc = " fn main () {"] # [doc = " # fn doc() {"] # [doc = "     let appender = tracing_appender::rolling::never(\"/some/path\", \"non-rolling.log\");"] # [doc = "     let (non_blocking_appender, _guard) = tracing_appender::non_blocking(appender);"] # [doc = ""] # [doc = "     let subscriber = tracing_subscriber::fmt().with_writer(non_blocking_appender);"] # [doc = ""] # [doc = "     tracing::subscriber::with_default(subscriber.finish(), || {"] # [doc = "         tracing::event!(tracing::Level::INFO, \"Hello\");"] # [doc = "     });"] # [doc = " # }"] # [doc = " }"] # [doc = " ```"] # [doc = ""] # [doc = " This will result in a log file located at `/some/path/non-rolling.log`."] pub fn never (directory : impl AsRef < Path > , file_name : impl AsRef < Path >) -> RollingFileAppender { RollingFileAppender :: new (Rotation :: NEVER , directory , file_name) }
};
}
