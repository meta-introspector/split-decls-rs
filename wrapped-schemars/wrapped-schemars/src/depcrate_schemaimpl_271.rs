// Generated macro for impl_271 (impl)
macro_rules! Depcrate_schemaimpl_271 {
() => {
// Module: crate::schema
// Provides: {"impl_271"}
// Dependencies: {}
impl < 'a > core :: convert :: TryFrom < & 'a mut Value > for & 'a mut Schema { type Error = serde_json :: Error ; fn try_from (value : & mut Value) -> serde_json :: Result < & mut Schema > { Schema :: validate (value) ? ; Ok (Schema :: ref_cast_mut (value)) } }
};
}
