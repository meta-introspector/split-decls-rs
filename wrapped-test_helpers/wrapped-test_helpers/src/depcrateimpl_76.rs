// Generated macro for impl_76 (impl)
macro_rules! Depcrateimpl_76 {
() => {
// Module: crate
// Provides: {"impl_76"}
// Dependencies: {}
impl < T : core :: fmt :: Debug + DefaultStrategy , const LANES : usize > DefaultStrategy for [T ; LANES] { type Strategy = crate :: array :: UniformArrayStrategy < T :: Strategy , Self > ; fn default_strategy () -> Self :: Strategy { Self :: Strategy :: new (T :: default_strategy ()) } }
};
}
