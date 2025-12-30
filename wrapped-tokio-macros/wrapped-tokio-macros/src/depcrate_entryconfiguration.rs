// Generated macro for Configuration (struct)
macro_rules! Depcrate_entryConfiguration {
() => {
// Module: crate::entry
// Provides: {"Configuration"}
// Dependencies: {}
struct Configuration { rt_multi_thread_available : bool , default_flavor : RuntimeFlavor , flavor : Option < RuntimeFlavor > , worker_threads : Option < (usize , Span) > , start_paused : Option < (bool , Span) > , is_test : bool , crate_name : Option < Path > , unhandled_panic : Option < (UnhandledPanic , Span) > , }
};
}
