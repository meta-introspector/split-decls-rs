// Generated macro for impl_269 (impl)
macro_rules! Depcrate_schemaimpl_269 {
() => {
// Module: crate::schema
// Provides: {"impl_269"}
// Dependencies: {}
impl core :: convert :: TryFrom < Value > for Schema { type Error = serde_json :: Error ; fn try_from (value : Value) -> serde_json :: Result < Schema > { Schema :: validate (& value) ? ; Ok (Schema (value)) } }
};
}
