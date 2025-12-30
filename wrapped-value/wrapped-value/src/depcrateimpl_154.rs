// Generated macro for impl_154 (impl)
macro_rules! Depcrateimpl_154 {
() => {
// Module: crate
// Provides: {"impl_154"}
// Dependencies: {}
impl TryFrom < Value > for serde_json :: Value { type Error = serde_json :: Error ; fn try_from (value : Value) -> Result < Self , Self :: Error > { serde_json :: to_value (value) } }
};
}
