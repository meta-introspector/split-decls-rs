// Generated macro for init_tracing (function)
macro_rules! Depcrateinit_tracing {
() => {
// Module: crate
// Provides: {"init_tracing"}
// Dependencies: {}
# [doc = " Initialize the tracing subscriber to log to a file"] # [doc = ""] # [doc = " This function initializes the tracing subscriber to log to a file named `tracing.log` in the"] # [doc = " current directory. The function returns a [`WorkerGuard`] that must be kept alive for the"] # [doc = " duration of the program to ensure that logs are flushed to the file on shutdown. The logs are"] # [doc = " written in a non-blocking fashion to ensure that the logs do not block the main thread."] fn init_tracing () -> Result < WorkerGuard > { let file = File :: create ("tracing.log") . wrap_err ("failed to create tracing.log") ? ; let (non_blocking , guard) = non_blocking (file) ; let env_filter = EnvFilter :: builder () . with_default_directive (Level :: DEBUG . into ()) . from_env_lossy () ; tracing_subscriber :: fmt () . with_writer (non_blocking) . with_env_filter (env_filter) . init () ; Ok (guard) }
};
}
