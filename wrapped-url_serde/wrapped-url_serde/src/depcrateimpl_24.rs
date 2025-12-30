// Generated macro for impl_24 (impl)
macro_rules! Depcrateimpl_24 {
() => {
// Module: crate
// Provides: {"impl_24"}
// Dependencies: {}
# [doc = " Deserializes this Option<URL> from a `serde` stream."] impl < 'de > Deserialize < 'de > for De < Option < Url > > { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > { let option_representation : Option < String > = Deserialize :: deserialize (deserializer) ? ; if let Some (s) = option_representation { return Url :: parse (& s) . map (Some) . map (De) . map_err (| err | { serde :: de :: Error :: custom (err . description ()) }) ; } Ok (De (None)) } }
};
}
