// Generated macro for xid (macro)
macro_rules! Depcrate_commonxid {
() => {
// Module: crate::common
// Provides: {"xid"}
// Dependencies: {}
macro_rules ! xid { ($ (# [$ outer : meta]) + $ name : ident , $ type : ty $ (, $ trait : ty) ?) => { # [cfg (any (feature = "system" , feature = "user"))] $ (# [$ outer]) + # [repr (transparent)] # [derive (Clone , PartialEq , Eq , Hash , PartialOrd , Ord , Debug)] pub struct $ name (pub (crate) $ type) ; # [cfg (any (feature = "system" , feature = "user"))] impl std :: ops :: Deref for $ name { type Target = $ type ; fn deref (& self) -> & Self :: Target { & self . 0 } } $ (# [cfg (any (feature = "system" , feature = "user"))] impl TryFrom < usize > for $ name { type Error = <$ type as TryFrom < usize >>:: Error ; fn try_from (t : usize) -> Result < Self , <$ type as TryFrom < usize >>:: Error > { Ok (Self (<$ type >:: try_from (t) ?)) } } # [cfg (any (feature = "system" , feature = "user"))] impl $ trait for $ name { type Err = <$ type as $ trait >:: Err ; fn from_str (t : & str) -> Result < Self , <$ type as $ trait >:: Err > { Ok (Self (<$ type >:: from_str (t) ?)) } }) ? } ; }
};
}
