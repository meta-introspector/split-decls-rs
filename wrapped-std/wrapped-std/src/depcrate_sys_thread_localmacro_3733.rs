// Generated macro for macro_3733 (macro)
macro_rules! Depcrate_sys_thread_localmacro_3733 {
() => {
// Module: crate::sys::thread_local
// Provides: {"macro_3733"}
// Dependencies: {}
cfg_select ! { any (all (target_family = "wasm" , not (target_feature = "atomics")) , target_os = "uefi" , target_os = "zkvm" , target_os = "trusty" ,) => { mod no_threads ; pub use no_threads :: { EagerStorage , LazyStorage , thread_local_inner } ; pub (crate) use no_threads :: { LocalPointer , local_pointer } ; } target_thread_local => { mod native ; pub use native :: { EagerStorage , LazyStorage , thread_local_inner } ; pub (crate) use native :: { LocalPointer , local_pointer } ; } _ => { mod os ; pub use os :: { Storage , thread_local_inner } ; pub (crate) use os :: { LocalPointer , local_pointer } ; } }
};
}
