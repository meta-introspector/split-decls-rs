// Generated macro for impl_798 (impl)
macro_rules! Depcrate_specimpl_798 {
() => {
// Module: crate::spec
// Provides: {"impl_798"}
// Dependencies: {}
# [doc = " `TargetOptions` being a separate type is basically an implementation detail of `Target` that is"] # [doc = " used for providing defaults. Perhaps there's a way to merge `TargetOptions` into `Target` so"] # [doc = " this `Deref` implementation is no longer necessary."] impl Deref for Target { type Target = TargetOptions ; # [inline] fn deref (& self) -> & Self :: Target { & self . options } }
};
}
