// Generated macro for impl_63 (impl)
macro_rules! Depcrateimpl_63 {
() => {
// Module: crate
// Provides: {"impl_63"}
// Dependencies: {}
impl < T : core :: fmt :: Debug + DefaultStrategy , const LANES : usize > DefaultStrategy for [T ; LANES] { type Strategy = crate :: array :: UniformArrayStrategy < T :: Strategy , Self > ; fn default_strategy () -> Self :: Strategy { Self :: Strategy :: new (T :: default_strategy ()) } }
};
}
