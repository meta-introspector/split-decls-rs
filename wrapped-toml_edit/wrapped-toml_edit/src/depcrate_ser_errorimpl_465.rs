// Generated macro for impl_465 (impl)
macro_rules! Depcrate_ser_errorimpl_465 {
() => {
// Module: crate::ser::error
// Provides: {"impl_465"}
// Dependencies: {}
impl serde_core :: ser :: Error for Error { fn custom < T > (msg : T) -> Self where T : std :: fmt :: Display , { Self :: custom (msg) } }
};
}
