// Generated macro for impl_174 (impl)
macro_rules! Depcrate_sharedimpl_174 {
() => {
// Module: crate::shared
// Provides: {"impl_174"}
// Dependencies: {}
impl < 'g , T > TryFrom < Ptr < 'g , T > > for Shared < T > { type Error = Ptr < 'g , T > ; # [inline] fn try_from (ptr : Ptr < 'g , T >) -> Result < Self , Self :: Error > { if let Some (shared) = ptr . get_shared () { Ok (shared) } else { Err (ptr) } } }
};
}
