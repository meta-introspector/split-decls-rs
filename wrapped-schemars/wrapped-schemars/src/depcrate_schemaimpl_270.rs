// Generated macro for impl_270 (impl)
macro_rules! Depcrate_schemaimpl_270 {
() => {
// Module: crate::schema
// Provides: {"impl_270"}
// Dependencies: {}
impl < 'a > core :: convert :: TryFrom < & 'a Value > for & 'a Schema { type Error = serde_json :: Error ; fn try_from (value : & Value) -> serde_json :: Result < & Schema > { Schema :: validate (value) ? ; Ok (Schema :: ref_cast (value)) } }
};
}
