// Generated macro for impl_61 (impl)
macro_rules! Depcrate_serimpl_61 {
() => {
// Module: crate::ser
// Provides: {"impl_61"}
// Dependencies: {}
impl serde_core :: ser :: Error for SerializerError { fn custom < T > (_msg : T) -> Self where T : core :: fmt :: Display , { Self :: InvalidProtocol } }
};
}
