// Generated macro for impl_397 (impl)
macro_rules! Depcrate_cldr_serde_week_dataimpl_397 {
() => {
// Module: crate::cldr_serde::week_data
// Provides: {"impl_397"}
// Dependencies: {}
impl < 'de > Deserialize < 'de > for Territory { fn deserialize < D > (deserializer : D) -> Result < Self , D :: Error > where D : Deserializer < 'de > , { struct TerritoryVisitor ; impl serde :: de :: Visitor < '_ > for TerritoryVisitor { type Value = Territory ; fn expecting (& self , formatter : & mut core :: fmt :: Formatter < '_ >) -> core :: fmt :: Result { write ! (formatter , "a valid Unicode Language Identifier or default territory literal") } fn visit_str < E > (self , s : & str) -> Result < Self :: Value , E > where E : serde :: de :: Error , { if let Some (prefix) = s . strip_suffix (ALT_VARIANT_SUFFIX) { return Ok (Territory :: AltVariantRegion (prefix . parse :: < Region > () . map_err (serde :: de :: Error :: custom) ? ,)) ; } Ok (Territory :: Region (s . parse :: < Region > () . map_err (serde :: de :: Error :: custom) ? ,)) } } deserializer . deserialize_string (TerritoryVisitor) } }
};
}
