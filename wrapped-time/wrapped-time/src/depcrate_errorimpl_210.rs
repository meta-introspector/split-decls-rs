// Generated macro for impl_210 (impl)
macro_rules! Depcrate_errorimpl_210 {
() => {
// Module: crate::error
// Provides: {"impl_210"}
// Dependencies: {}
impl fmt :: Display for Error { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: ConversionRange (e) => e . fmt (f) , Self :: ComponentRange (e) => e . fmt (f) , # [cfg (feature = "local-offset")] Self :: IndeterminateOffset (e) => e . fmt (f) , # [cfg (feature = "formatting")] Self :: Format (e) => e . fmt (f) , # [cfg (feature = "parsing")] Self :: ParseFromDescription (e) => e . fmt (f) , # [cfg (feature = "parsing")] # [allow (deprecated)] Self :: UnexpectedTrailingCharacters { never } => match * never { } , # [cfg (feature = "parsing")] Self :: TryFromParsed (e) => e . fmt (f) , # [cfg (all (any (feature = "formatting" , feature = "parsing") , feature = "alloc"))] Self :: InvalidFormatDescription (e) => e . fmt (f) , Self :: DifferentVariant (e) => e . fmt (f) , Self :: InvalidVariant (e) => e . fmt (f) , } } }
};
}
