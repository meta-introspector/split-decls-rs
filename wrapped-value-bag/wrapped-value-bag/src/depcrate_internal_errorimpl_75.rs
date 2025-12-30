// Generated macro for impl_75 (impl)
macro_rules! Depcrate_internal_errorimpl_75 {
() => {
// Module: crate::internal::error
// Provides: {"impl_75"}
// Dependencies: {}
impl < 'v > ValueBag < 'v > { # [doc = " Get a value from an error."] pub fn capture_error < T > (value : & 'v T) -> Self where T : error :: Error + 'static , { ValueBag { inner : Internal :: Error (value) , } } # [doc = " Get a value from an erased value."] # [inline] pub const fn from_dyn_error (value : & 'v (dyn error :: Error + 'static)) -> Self { ValueBag { inner : Internal :: AnonError (value) , } } # [doc = " Try get an error from this value."] # [inline] pub fn to_borrowed_error (& self) -> Option < & 'v (dyn Error + 'static) > { match self . inner { Internal :: Error (value) => Some (value . as_super ()) , Internal :: AnonError (value) => Some (value) , _ => None , } } }
};
}
