// Generated macro for impl_384 (impl)
macro_rules! Depcrate_de_errorimpl_384 {
() => {
// Module: crate::de::error
// Provides: {"impl_384"}
// Dependencies: {}
impl serde_core :: de :: Error for Error { fn custom < T > (msg : T) -> Self where T : std :: fmt :: Display , { Self :: custom (msg , None) } }
};
}
