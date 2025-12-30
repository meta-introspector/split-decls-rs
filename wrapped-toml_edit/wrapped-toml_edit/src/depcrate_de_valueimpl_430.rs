// Generated macro for impl_430 (impl)
macro_rules! Depcrate_de_valueimpl_430 {
() => {
// Module: crate::de::value
// Provides: {"impl_430"}
// Dependencies: {}
# [cfg (feature = "parse")] impl std :: str :: FromStr for ValueDeserializer { type Err = Error ; # [doc = " Parses a value from a &str"] fn from_str (s : & str) -> Result < Self , Self :: Err > { let value = s . parse :: < crate :: Value > () . map_err (Error :: from) ? ; Ok (value . into_deserializer ()) } }
};
}
