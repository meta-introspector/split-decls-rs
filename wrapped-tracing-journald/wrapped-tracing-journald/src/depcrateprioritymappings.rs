// Generated macro for PriorityMappings (struct)
macro_rules! DepcratePriorityMappings {
() => {
// Module: crate
// Provides: {"PriorityMappings"}
// Dependencies: {}
# [doc = " Mappings from tracing [`Level`]s to journald [priorities]."] # [doc = ""] # [doc = " [priorities]: Priority"] # [derive (Debug , Clone)] pub struct PriorityMappings { # [doc = " Priority mapped to the `ERROR` level"] pub error : Priority , # [doc = " Priority mapped to the `WARN` level"] pub warn : Priority , # [doc = " Priority mapped to the `INFO` level"] pub info : Priority , # [doc = " Priority mapped to the `DEBUG` level"] pub debug : Priority , # [doc = " Priority mapped to the `TRACE` level"] pub trace : Priority , }
};
}
