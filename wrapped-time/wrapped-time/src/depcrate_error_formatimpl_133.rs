// Generated macro for impl_133 (impl)
macro_rules! Depcrate_error_formatimpl_133 {
() => {
// Module: crate::error::format
// Provides: {"impl_133"}
// Dependencies: {}
impl fmt :: Display for Format { # [inline] fn fmt (& self , f : & mut fmt :: Formatter < '_ >) -> fmt :: Result { match self { Self :: InsufficientTypeInformation => f . write_str ("The type being formatted does not contain sufficient information to format a \
                 component." ,) , Self :: InvalidComponent (component) => write ! (f , "The {component} component cannot be formatted into the requested format.") , Self :: ComponentRange (err) => err . fmt (f) , Self :: StdIo (err) => err . fmt (f) , } } }
};
}
