// Generated macro for capture_quiche_logs (function)
macro_rules! Depcratecapture_quiche_logs {
() => {
// Module: crate
// Provides: {"capture_quiche_logs"}
// Dependencies: {}
# [doc = " Forward Quiche logs into the slog::Drain currently used by Foundations"] # [doc = ""] # [doc = " # Warning"] # [doc = ""] # [doc = " This should **only be used for local debugging**. Quiche can potentially"] # [doc = " emit lots (and lots, and lots) of logs (the TRACE level emits a log record"] # [doc = " on every packet and frame) and you can very easily overwhelm your logging"] # [doc = " pipeline."] # [doc = ""] # [doc = " # Note"] # [doc = ""] # [doc = " Quiche uses the `env_logger` crate, which uses `log` under the hood. `log`"] # [doc = " requires that you only set the global logger once. That means that we have"] # [doc = " to register the logger at `listen()` time for servers - for clients, we"] # [doc = " should register loggers when the `quiche::Connection` is established."] pub (crate) fn capture_quiche_logs () { GLOBAL_LOGGER_ONCE . call_once (| | { use foundations :: telemetry :: log as foundations_log ; use log :: Level as std_level ; let curr_logger = Arc :: clone (& foundations_log :: slog_logger ()) . read () . clone () ; let scope_guard = slog_scope :: set_global_logger (curr_logger) ; let normalized_level = match foundations_log :: verbosity () { LogVerbosity :: Critical | LogVerbosity :: Error => std_level :: Error , LogVerbosity :: Warning => std_level :: Warn , LogVerbosity :: Info => std_level :: Info , LogVerbosity :: Debug => std_level :: Debug , LogVerbosity :: Trace => std_level :: Trace , } ; slog_stdlog :: init_with_level (normalized_level) . unwrap () ; std :: mem :: forget (scope_guard) }) ; }
};
}
