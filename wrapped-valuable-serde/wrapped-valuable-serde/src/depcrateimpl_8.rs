// Generated macro for impl_8 (impl)
macro_rules! Depcrateimpl_8 {
() => {
// Module: crate
// Provides: {"impl_8"}
// Dependencies: {}
impl < V > Valuable for Serializable < V > where V : Valuable , { fn as_value (& self) -> Value < '_ > { self . 0 . as_value () } fn visit (& self , visit : & mut dyn Visit) { self . 0 . visit (visit) ; } }
};
}
