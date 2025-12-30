// Generated macro for RollingWriter (struct)
macro_rules! Depcrate_rollingRollingWriter {
() => {
// Module: crate::rolling
// Provides: {"RollingWriter"}
// Dependencies: {}
# [doc = " A [writer] that writes to a rolling log file."] # [doc = ""] # [doc = " This is returned by the [`MakeWriter`] implementation for [`RollingFileAppender`]."] # [doc = ""] # [doc = " [writer]: std::io::Write"] # [doc = " [`MakeWriter`]: tracing_subscriber::fmt::writer::MakeWriter"] # [derive (Debug)] pub struct RollingWriter < 'a > (RwLockReadGuard < 'a , File >) ;
};
}
