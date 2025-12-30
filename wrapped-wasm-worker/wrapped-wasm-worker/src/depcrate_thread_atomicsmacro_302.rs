// Generated macro for macro_302 (macro)
macro_rules! Depcrate_thread_atomicsmacro_302 {
() => {
// Module: crate::thread::atomics
// Provides: {"macro_302"}
// Dependencies: {}
thread_local ! { # [doc = " [`Memory`] of the Wasm module."] pub (super) static MEMORY : Memory = wasm_bindgen :: memory () . unchecked_into () ; # [doc = " [`Memory`] of the Wasm module as a [`Int32Array`]."] pub (super) static MEMORY_ARRAY : Int32Array = Int32Array :: new (& MEMORY . with (Memory :: buffer)) ; # [doc = " Wasm [`Module`]."] pub (super) static MODULE : Module = wasm_bindgen :: module () . unchecked_into () ; }
};
}
