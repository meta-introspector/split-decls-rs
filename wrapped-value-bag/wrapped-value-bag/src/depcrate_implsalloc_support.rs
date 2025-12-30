// Generated macro for alloc_support (module)
macro_rules! Depcrate_implsalloc_support {
() => {
// Module: crate::impls
// Provides: {"alloc_support"}
// Dependencies: {}
# [cfg (feature = "alloc")] mod alloc_support { use super :: * ; use crate :: std :: { borrow :: Cow , string :: String } ; impl < 'v > From < & 'v String > for ValueBag < 'v > { # [inline] fn from (v : & 'v String) -> Self { ValueBag :: from_str (v) } } impl < 'v > TryFrom < ValueBag < 'v > > for String { type Error = Error ; # [inline] fn try_from (v : ValueBag < 'v >) -> Result < Self , Error > { Ok (v . to_str () . ok_or_else (| | Error :: msg ("conversion failed")) ? . into_owned ()) } } impl < 'v > From < & 'v Cow < 'v , str > > for ValueBag < 'v > { # [inline] fn from (v : & 'v Cow < 'v , str >) -> Self { ValueBag :: from_str (v) } } impl < 'v > TryFrom < ValueBag < 'v > > for Cow < 'v , str > { type Error = Error ; # [inline] fn try_from (v : ValueBag < 'v >) -> Result < Self , Error > { v . to_str () . ok_or_else (| | Error :: msg ("conversion failed")) } } }
};
}
