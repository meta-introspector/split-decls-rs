// Generated macro for owned (module)
macro_rules! Depcrate_internal_errorowned {
() => {
// Module: crate::internal::error
// Provides: {"owned"}
// Dependencies: {}
# [cfg (feature = "owned")] pub (crate) mod owned { use crate :: std :: { boxed :: Box , error , fmt , string :: ToString } ; # [derive (Clone , Debug)] pub (crate) struct OwnedError { display : Box < str > , source : Option < Box < OwnedError > > , } impl fmt :: Display for OwnedError { fn fmt (& self , f : & mut fmt :: Formatter) -> fmt :: Result { fmt :: Display :: fmt (& self . display , f) } } impl error :: Error for OwnedError { fn source (& self) -> Option < & (dyn error :: Error + 'static) > { if let Some (ref source) = self . source { Some (& * * source) } else { None } } } pub (crate) fn buffer (err : & (dyn error :: Error + 'static)) -> OwnedError { OwnedError { display : err . to_string () . into () , source : err . source () . map (buffer) . map (Box :: new) , } } }
};
}
