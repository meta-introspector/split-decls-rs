// Generated macro for macro_457 (macro)
macro_rules! Depcrate_thread_unsupportedmacro_457 {
() => {
// Module: crate::thread::unsupported
// Provides: {"macro_457"}
// Dependencies: {}
thread_local ! { static ZERO_ARRAY : Int32Array = { if super :: has_shared_array_buffer_support () { Int32Array :: new (& SharedArrayBuffer :: new (4)) } else { let descriptor : MemoryDescriptor = Object :: new () . unchecked_into () ; descriptor . set_initial (1) ; descriptor . set_maximum (1) ; descriptor . set_shared (true) ; let memory = Memory :: new (& descriptor) . expect ("`new Memory` is not expected to fail") ; Int32Array :: new (& memory . buffer ()) } } ; }
};
}
