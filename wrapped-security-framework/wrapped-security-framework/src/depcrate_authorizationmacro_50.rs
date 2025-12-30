// Generated macro for macro_50 (macro)
macro_rules! Depcrate_authorizationmacro_50 {
() => {
// Module: crate::authorization
// Provides: {"macro_50"}
// Dependencies: {}
bitflags :: bitflags ! { # [doc = " The flags used to specify authorization options."] # [derive (Debug , Clone , Copy , PartialEq , Eq , Hash)] pub struct Flags : sys :: AuthorizationFlags { # [doc = " An empty flag set that you use as a placeholder when you don't want"] # [doc = " any of the other flags."] const DEFAULTS = sys :: kAuthorizationFlagDefaults ; # [doc = " A flag that permits user interaction as needed."] const INTERACTION_ALLOWED = sys :: kAuthorizationFlagInteractionAllowed ; # [doc = " A flag that permits the Security Server to attempt to grant the"] # [doc = " rights requested."] const EXTEND_RIGHTS = sys :: kAuthorizationFlagExtendRights ; # [doc = " A flag that permits the Security Server to grant rights on an"] # [doc = " individual basis."] const PARTIAL_RIGHTS = sys :: kAuthorizationFlagPartialRights ; # [doc = " A flag that instructs the Security Server to revoke authorization."] const DESTROY_RIGHTS = sys :: kAuthorizationFlagDestroyRights ; # [doc = " A flag that instructs the Security Server to preauthorize the rights"] # [doc = " requested."] const PREAUTHORIZE = sys :: kAuthorizationFlagPreAuthorize ; } }
};
}
