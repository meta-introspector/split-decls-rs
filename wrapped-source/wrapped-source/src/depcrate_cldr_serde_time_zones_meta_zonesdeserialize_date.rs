// Generated macro for deserialize_date (function)
macro_rules! Depcrate_cldr_serde_time_zones_meta_zonesdeserialize_date {
() => {
// Module: crate::cldr_serde::time_zones::meta_zones
// Provides: {"deserialize_date"}
// Dependencies: {}
fn deserialize_date < 'de , D : serde :: de :: Deserializer < 'de > > (deserializer : D ,) -> Result < Option < Timestamp > , D :: Error > { use icu :: calendar :: Iso ; use icu :: time :: zone :: UtcOffset ; use icu :: time :: DateTime ; use serde :: de :: Error ; let Some (timestamp) = Option :: < String > :: deserialize (deserializer) ? else { return Ok (None) ; } ; let DateTime { date , mut time } = DateTime :: try_from_str (& timestamp , Iso) . map_err (| _ | D :: Error :: custom ("Invalid metazone timestamp")) ? ; if time == Time :: try_new (0 , 45 , 0 , 0) . unwrap () { time = Time :: try_new (0 , 44 , 30 , 0) . unwrap () } Ok (Some (Timestamp { date , time , zone : UtcOffset :: zero () , })) }
};
}
