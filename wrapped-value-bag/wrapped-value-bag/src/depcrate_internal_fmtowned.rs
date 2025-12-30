// Generated macro for owned (module)
macro_rules! Depcrate_internal_fmtowned {
() => {
// Module: crate::internal::fmt
// Provides: {"owned"}
// Dependencies: {}
# [cfg (feature = "owned")] pub (crate) mod owned { use crate :: std :: { boxed :: Box , fmt , string :: ToString } ; impl fmt :: Debug for crate :: OwnedValueBag { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { fmt :: Debug :: fmt (& self . by_ref () , f) } } impl fmt :: Display for crate :: OwnedValueBag { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { fmt :: Display :: fmt (& self . by_ref () , f) } } # [derive (Clone)] pub (crate) struct OwnedFmt (Box < str >) ; pub (crate) fn buffer_debug (v : impl fmt :: Debug) -> OwnedFmt { OwnedFmt (format ! ("{:?}" , v) . into ()) } pub (crate) fn buffer_display (v : impl fmt :: Display) -> OwnedFmt { OwnedFmt (v . to_string () . into ()) } impl fmt :: Debug for OwnedFmt { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { fmt :: Display :: fmt (self , f) } } impl fmt :: Display for OwnedFmt { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { fmt :: Display :: fmt (& self . 0 , f) } } }
};
}
