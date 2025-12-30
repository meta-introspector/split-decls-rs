// Generated macro for impl_44 (impl)
macro_rules! Depcrateimpl_44 {
() => {
// Module: crate
// Provides: {"impl_44"}
// Dependencies: {}
impl PriorityMappings { # [doc = " Returns the default priority mappings:"] # [doc = ""] # [doc = " - [`tracing::Level::ERROR`][]: [`Priority::Error`] (3)"] # [doc = " - [`tracing::Level::WARN`][]: [`Priority::Warning`] (4)"] # [doc = " - [`tracing::Level::INFO`][]: [`Priority::Notice`] (5)"] # [doc = " - [`tracing::Level::DEBUG`][]: [`Priority::Informational`] (6)"] # [doc = " - [`tracing::Level::TRACE`][]: [`Priority::Debug`] (7)"] # [doc = ""] # [doc = " [`tracing::Level::ERROR`]: tracing_core::Level::ERROR"] # [doc = " [`tracing::Level::WARN`]: tracing_core::Level::WARN"] # [doc = " [`tracing::Level::INFO`]: tracing_core::Level::INFO"] # [doc = " [`tracing::Level::DEBUG`]: tracing_core::Level::DEBUG"] # [doc = " [`tracing::Level::TRACE`]: tracing_core::Level::TRACE"] pub fn new () -> PriorityMappings { Self { error : Priority :: Error , warn : Priority :: Warning , info : Priority :: Notice , debug : Priority :: Informational , trace : Priority :: Debug , } } }
};
}
