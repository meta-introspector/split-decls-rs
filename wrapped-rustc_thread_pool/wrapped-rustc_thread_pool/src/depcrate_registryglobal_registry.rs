// Generated macro for global_registry (function)
macro_rules! Depcrate_registryglobal_registry {
() => {
// Module: crate::registry
// Provides: {"global_registry"}
// Dependencies: {}
# [doc = " Starts the worker threads (if that has not already happened). If"] # [doc = " initialization has not already occurred, use the default"] # [doc = " configuration."] pub (super) fn global_registry () -> & 'static Arc < Registry > { set_global_registry (default_global_registry) . or_else (| err | { debug_assert ! (THE_REGISTRY_SET . is_completed ()) ; let the_registry = unsafe { & * ptr :: addr_of ! (THE_REGISTRY) } ; the_registry . as_ref () . ok_or (err) }) . expect ("The global thread pool has not been initialized.") }
};
}
