// Generated macro for AuthorizationItemSetStorage (struct)
macro_rules! Depcrate_authorizationAuthorizationItemSetStorage {
() => {
// Module: crate::authorization
// Provides: {"AuthorizationItemSetStorage"}
// Dependencies: {}
# [doc = " Used by `AuthorizationItemSetBuilder` to store data pointed to by"] # [doc = " `sys::AuthorizationItemSet`."] # [derive (Debug)] pub struct AuthorizationItemSetStorage { # [doc = " The layout of this is a little awkward because of the requirements of"] # [doc = " Apple's APIs. `items` contains pointers to data owned by `names` and"] # [doc = " `values`, so we must not modify them once `items` has been set up."] names : Vec < CString > , values : Vec < Option < Vec < u8 > > > , items : Vec < sys :: AuthorizationItem > , # [doc = " Must not be given to APIs which would attempt to modify it."] # [doc = ""] # [doc = " See `AuthorizationItemSet` for sets owned by the Security Server which"] # [doc = " are writable."] pub set : sys :: AuthorizationItemSet , }
};
}
