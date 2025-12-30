// Generated macro for impl_3367 (impl)
macro_rules! Depcrate_sync_once_lockimpl_3367 {
() => {
// Module: crate::sync::once_lock
// Provides: {"impl_3367"}
// Dependencies: {}
# [stable (feature = "once_cell" , since = "1.70.0")] impl < T > From < T > for OnceLock < T > { # [doc = " Creates a new cell with its contents set to `value`."] # [doc = ""] # [doc = " # Example"] # [doc = ""] # [doc = " ```"] # [doc = " use std::sync::OnceLock;"] # [doc = ""] # [doc = " # fn main() -> Result<(), i32> {"] # [doc = " let a = OnceLock::from(3);"] # [doc = " let b = OnceLock::new();"] # [doc = " b.set(3)?;"] # [doc = " assert_eq!(a, b);"] # [doc = " Ok(())"] # [doc = " # }"] # [doc = " ```"] # [inline] fn from (value : T) -> Self { let cell = Self :: new () ; match cell . set (value) { Ok (()) => cell , Err (_) => unreachable ! () , } } }
};
}
