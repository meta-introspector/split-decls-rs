// Generated macro for ExtendValue (trait)
macro_rules! Depcrate_internal_seqExtendValue {
() => {
// Module: crate::internal::seq
// Provides: {"ExtendValue"}
// Dependencies: {}
# [allow (dead_code)] pub (crate) trait ExtendValue < 'v > { fn extend (& mut self , v : Internal) ; fn extend_borrowed (& mut self , v : Internal < 'v >) { self . extend (v) ; } }
};
}
