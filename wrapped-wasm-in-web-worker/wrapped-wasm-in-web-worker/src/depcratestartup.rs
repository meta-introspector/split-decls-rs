// Generated macro for startup (function)
macro_rules! Depcratestartup {
() => {
// Module: crate
// Provides: {"startup"}
// Dependencies: {}
# [doc = " Run entry point for the main thread."] # [wasm_bindgen] pub fn startup () { let worker_handle = Rc :: new (RefCell :: new (Worker :: new ("./worker.js") . unwrap ())) ; console :: log_1 (& "Created a new worker from within Wasm" . into ()) ; setup_input_oninput_callback (worker_handle) ; }
};
}
