// Generated macro for macro_350 (macro)
macro_rules! Depcrate_thread_globalmacro_350 {
() => {
// Module: crate::thread::global
// Provides: {"macro_350"}
// Dependencies: {}
thread_local ! { static GLOBAL : Global = { let global : GlobalExt = js_sys :: global () . unchecked_into () ; if ! global . window () . is_undefined () { Global :: Window (global . unchecked_into ()) } else if ! global . dedicated_worker_global_scope () . is_undefined () { Global :: Dedicated (global . unchecked_into ()) } else if ! global . shared_worker_global_scope () . is_undefined () { Global :: Shared (global . unchecked_into ()) } else if ! global . service_worker_global_scope () . is_undefined () { Global :: Service (global . unchecked_into ()) } else if ! global . worklet_global_scope () . is_undefined () { Global :: Worklet } else if ! global . worker_global_scope () . is_undefined () { Global :: Worker (global . unchecked_into ()) } else { Global :: Unknown } } ; }
};
}
