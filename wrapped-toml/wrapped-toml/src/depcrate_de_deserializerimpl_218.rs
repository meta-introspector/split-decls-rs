// Generated macro for impl_218 (impl)
macro_rules! Depcrate_de_deserializerimpl_218 {
() => {
// Module: crate::de::deserializer
// Provides: {"impl_218"}
// Dependencies: {}
impl < 'i > Deserializer < 'i > { # [doc = " Parse a TOML document"] pub fn parse (raw : & 'i str) -> Result < Self , Error > { let root = DeTable :: parse (raw) ? ; let span = root . span () ; let root = root . into_inner () ; Ok (Self { span , root , raw : Some (raw) , }) } # [doc = " Deprecated, replaced with [`Deserializer::parse`]"] # [deprecated (since = "0.9.0" , note = "replaced with `Deserializer::parse`")] pub fn new (raw : & 'i str) -> Result < Self , Error > { Self :: parse (raw) } fn into_table_de (self) -> ValueDeserializer < 'i > { ValueDeserializer :: with_parts (DeValue :: Table (self . root) , self . span) } }
};
}
