// Generated macro for impl_62 (impl)
macro_rules! Depcrate_authorizationimpl_62 {
() => {
// Module: crate::authorization
// Provides: {"impl_62"}
// Dependencies: {}
impl TryFrom < AuthorizationExternalForm > for Authorization { type Error = Error ; # [doc = " Internalizes the external representation of an authorization reference."] # [cold] fn try_from (external_form : AuthorizationExternalForm) -> Result < Self > { let mut handle = MaybeUninit :: < sys :: AuthorizationRef > :: uninit () ; let status = unsafe { sys :: AuthorizationCreateFromExternalForm (& external_form , handle . as_mut_ptr ()) } ; if status != sys :: errAuthorizationSuccess { return Err (Error :: from_code (status)) ; } let auth = Self { handle : unsafe { handle . assume_init () } , free_flags : Flags :: default () , } ; Ok (auth) } }
};
}
