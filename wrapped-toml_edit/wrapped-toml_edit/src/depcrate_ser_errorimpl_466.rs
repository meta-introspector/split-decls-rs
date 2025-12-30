// Generated macro for impl_466 (impl)
macro_rules! Depcrate_ser_errorimpl_466 {
() => {
// Module: crate::ser::error
// Provides: {"impl_466"}
// Dependencies: {}
impl std :: fmt :: Display for Error { fn fmt (& self , formatter : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { match self { Self :: UnsupportedType (Some (t)) => write ! (formatter , "unsupported {t} type") , Self :: UnsupportedType (None) => write ! (formatter , "unsupported rust type") , Self :: OutOfRange (Some (t)) => write ! (formatter , "out-of-range value for {t} type") , Self :: OutOfRange (None) => write ! (formatter , "out-of-range value") , Self :: UnsupportedNone => "unsupported None value" . fmt (formatter) , Self :: KeyNotString => "map key was not a string" . fmt (formatter) , Self :: DateInvalid => "a serialized date was invalid" . fmt (formatter) , Self :: Custom (s) => s . fmt (formatter) , } } }
};
}
