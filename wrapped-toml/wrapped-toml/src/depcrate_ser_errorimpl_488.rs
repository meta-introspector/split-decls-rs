// Generated macro for impl_488 (impl)
macro_rules! Depcrate_ser_errorimpl_488 {
() => {
// Module: crate::ser::error
// Provides: {"impl_488"}
// Dependencies: {}
impl serde_core :: ser :: Error for Error { fn custom < T > (msg : T) -> Self where T : core :: fmt :: Display , { Self :: new (msg) } }
};
}
