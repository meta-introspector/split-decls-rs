// Generated macro for impl_125 (impl)
macro_rules! Depcrate_dispatcherimpl_125 {
() => {
// Module: crate::dispatcher
// Provides: {"impl_125"}
// Dependencies: {}
# [cfg (feature = "std")] impl < 'a > Entered < 'a > { # [inline] fn current (& self) -> Ref < 'a , Dispatch > { let default = self . 0 . default . borrow () ; Ref :: map (default , | default | match default { Some (default) => default , None => get_global () , }) } }
};
}
