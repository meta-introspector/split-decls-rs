// Generated macro for impl_361 (impl)
macro_rules! Depcrate_keyimpl_361 {
() => {
// Module: crate::key
// Provides: {"impl_361"}
// Dependencies: {}
impl DatabaseKeyIndex { # [inline] pub (crate) const fn new (ingredient_index : IngredientIndex , key_index : Id) -> Self { Self { key_index , ingredient_index , } } pub const fn ingredient_index (self) -> IngredientIndex { self . ingredient_index } pub const fn key_index (self) -> Id { self . key_index } pub (crate) fn maybe_changed_after (& self , db : crate :: database :: RawDatabase < '_ > , zalsa : & Zalsa , last_verified_at : crate :: Revision , cycle_heads : & mut VerifyCycleHeads ,) -> VerifyResult { unsafe { zalsa . lookup_ingredient (self . ingredient_index ()) . maybe_changed_after (zalsa , db , self . key_index () , last_verified_at , cycle_heads) } } pub (crate) fn remove_stale_output (& self , zalsa : & Zalsa , executor : DatabaseKeyIndex) { zalsa . lookup_ingredient (self . ingredient_index ()) . remove_stale_output (zalsa , executor , self . key_index ()) } pub (crate) fn mark_validated_output (& self , zalsa : & Zalsa , database_key_index : DatabaseKeyIndex ,) { zalsa . lookup_ingredient (self . ingredient_index ()) . mark_validated_output (zalsa , database_key_index , self . key_index ()) } }
};
}
