// Generated macro for impl_227 (impl)
macro_rules! Depcrate_de_errorimpl_227 {
() => {
// Module: crate::de::error
// Provides: {"impl_227"}
// Dependencies: {}
# [cfg (feature = "serde")] impl serde_core :: de :: Error for Error { fn custom < T > (msg : T) -> Self where T : core :: fmt :: Display , { Self :: custom (msg . to_string () , None) } }
};
}
