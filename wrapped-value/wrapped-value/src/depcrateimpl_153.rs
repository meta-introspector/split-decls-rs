// Generated macro for impl_153 (impl)
macro_rules! Depcrateimpl_153 {
() => {
// Module: crate
// Provides: {"impl_153"}
// Dependencies: {}
impl TryFrom < serde_json :: Value > for Value { type Error = serde_json :: Error ; fn try_from (value : serde_json :: Value) -> Result < Self , Self :: Error > { Self :: deserialize (value) } }
};
}
