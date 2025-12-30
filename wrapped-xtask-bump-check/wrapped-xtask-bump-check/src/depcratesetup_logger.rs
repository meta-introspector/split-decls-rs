// Generated macro for setup_logger (function)
macro_rules! Depcratesetup_logger {
() => {
// Module: crate
// Provides: {"setup_logger"}
// Dependencies: {}
fn setup_logger () { let env = tracing_subscriber :: EnvFilter :: from_env ("CARGO_LOG") ; tracing_subscriber :: fmt () . with_timer (tracing_subscriber :: fmt :: time :: Uptime :: default ()) . with_ansi (std :: io :: IsTerminal :: is_terminal (& std :: io :: stderr ())) . with_writer (std :: io :: stderr) . with_env_filter (env) . init () ; }
};
}
