// Generated macro for impl_146 (impl)
macro_rules! Depcrateimpl_146 {
() => {
// Module: crate
// Provides: {"impl_146"}
// Dependencies: {}
impl TryFrom < serde_json :: Value > for ConstValue { type Error = serde_json :: Error ; fn try_from (value : serde_json :: Value) -> Result < Self , Self :: Error > { Self :: deserialize (value) } }
};
}
