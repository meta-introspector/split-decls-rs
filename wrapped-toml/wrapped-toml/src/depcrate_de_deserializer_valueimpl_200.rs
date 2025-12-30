// Generated macro for impl_200 (impl)
macro_rules! Depcrate_de_deserializer_valueimpl_200 {
() => {
// Module: crate::de::deserializer::value
// Provides: {"impl_200"}
// Dependencies: {}
impl < 'i > ValueDeserializer < 'i > { # [doc = " Parse a TOML value"] pub fn parse (raw : & 'i str) -> Result < Self , Error > { let input = DeValue :: parse (raw) ? ; let span = input . span () ; let input = input . into_inner () ; Ok (Self :: with_parts (input , span)) } # [doc = " Deprecated, replaced with [`ValueDeserializer::parse`]"] # [deprecated (since = "0.9.0" , note = "replaced with `ValueDeserializer::parse`")] pub fn new (raw : & 'i str) -> Result < Self , Error > { Self :: parse (raw) } pub (crate) fn with_parts (input : DeValue < 'i > , span : core :: ops :: Range < usize >) -> Self { Self { input , span , validate_struct_keys : false , } } pub (crate) fn with_struct_key_validation (mut self) -> Self { self . validate_struct_keys = true ; self } }
};
}
