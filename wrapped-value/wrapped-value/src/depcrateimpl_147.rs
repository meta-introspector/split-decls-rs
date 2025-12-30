// Generated macro for impl_147 (impl)
macro_rules! Depcrateimpl_147 {
() => {
// Module: crate
// Provides: {"impl_147"}
// Dependencies: {}
impl TryFrom < ConstValue > for serde_json :: Value { type Error = serde_json :: Error ; fn try_from (value : ConstValue) -> Result < Self , Self :: Error > { serde_json :: to_value (value) } }
};
}
