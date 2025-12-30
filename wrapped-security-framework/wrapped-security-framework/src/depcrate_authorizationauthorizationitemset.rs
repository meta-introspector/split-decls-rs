// Generated macro for AuthorizationItemSet (struct)
macro_rules! Depcrate_authorizationAuthorizationItemSet {
() => {
// Module: crate::authorization
// Provides: {"AuthorizationItemSet"}
// Dependencies: {}
# [doc = " A set of authorization items returned and owned by the Security Server."] # [derive (Debug)] # [repr (C)] pub struct AuthorizationItemSet < 'a > { inner : * const sys :: AuthorizationItemSet , phantom : PhantomData < & 'a sys :: AuthorizationItemSet > , }
};
}
