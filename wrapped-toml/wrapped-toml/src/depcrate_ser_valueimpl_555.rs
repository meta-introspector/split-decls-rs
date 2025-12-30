// Generated macro for impl_555 (impl)
macro_rules! Depcrate_ser_valueimpl_555 {
() => {
// Module: crate::ser::value
// Provides: {"impl_555"}
// Dependencies: {}
impl < 'd > ValueSerializer < 'd > { # [doc = " Creates a new serializer which will emit TOML into the buffer provided."] # [doc = ""] # [doc = " The serializer can then be used to serialize a type after which the data"] # [doc = " will be present in `dst`."] pub fn new (dst : & 'd mut String) -> Self { Self { dst , style : Default :: default () , } } pub (crate) fn with_style (dst : & 'd mut String , style : Style) -> Self { Self { dst , style } } }
};
}
