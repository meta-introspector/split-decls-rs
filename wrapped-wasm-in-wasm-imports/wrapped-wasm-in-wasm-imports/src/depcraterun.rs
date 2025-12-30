// Generated macro for run (function)
macro_rules! Depcraterun {
() => {
// Module: crate
// Provides: {"run"}
// Dependencies: {}
# [wasm_bindgen (start)] fn run () { spawn_local (async { match run_async () . await { Ok (_) => console_log ! ("Finished") , Err (e) => console_error ! ("{:?}" , e) , } }) ; }
};
}
