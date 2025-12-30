// Generated macro for impl_211 (impl)
macro_rules! Depcrate_errorimpl_211 {
() => {
// Module: crate::error
// Provides: {"impl_211"}
// Dependencies: {}
impl core :: error :: Error for Error { # [inline] fn source (& self) -> Option < & (dyn core :: error :: Error + 'static) > { match self { Self :: ConversionRange (err) => Some (err) , Self :: ComponentRange (err) => Some (err) , # [cfg (feature = "local-offset")] Self :: IndeterminateOffset (err) => Some (err) , # [cfg (feature = "formatting")] Self :: Format (err) => Some (err) , # [cfg (feature = "parsing")] Self :: ParseFromDescription (err) => Some (err) , # [cfg (feature = "parsing")] # [allow (deprecated)] Self :: UnexpectedTrailingCharacters { never } => match * never { } , # [cfg (feature = "parsing")] Self :: TryFromParsed (err) => Some (err) , # [cfg (all (any (feature = "formatting" , feature = "parsing") , feature = "alloc"))] Self :: InvalidFormatDescription (err) => Some (err) , Self :: DifferentVariant (err) => Some (err) , Self :: InvalidVariant (err) => Some (err) , } } }
};
}
