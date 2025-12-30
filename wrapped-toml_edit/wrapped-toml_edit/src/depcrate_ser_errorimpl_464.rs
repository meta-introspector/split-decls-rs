// Generated macro for impl_464 (impl)
macro_rules! Depcrate_ser_errorimpl_464 {
() => {
// Module: crate::ser::error
// Provides: {"impl_464"}
// Dependencies: {}
impl Error { pub (crate) fn custom < T > (msg : T) -> Self where T : std :: fmt :: Display , { Self :: Custom (msg . to_string ()) } pub (crate) fn unsupported_type (t : Option < & 'static str >) -> Self { Self :: UnsupportedType (t) } pub (crate) fn out_of_range (t : Option < & 'static str >) -> Self { Self :: OutOfRange (t) } pub (crate) fn unsupported_none () -> Self { Self :: UnsupportedNone } pub (crate) fn key_not_string () -> Self { Self :: KeyNotString } pub (crate) fn date_invalid () -> Self { Self :: DateInvalid } }
};
}
