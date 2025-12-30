// Generated macro for impl_147 (impl)
macro_rules! Depcrate_dataimpl_147 {
() => {
// Module: crate::data
// Provides: {"impl_147"}
// Dependencies: {}
# [cfg (feature = "json")] impl < S : serde :: Serialize > IntoJson for S { fn into_json (self) -> Data { match serde_json :: to_value (self) { Ok (value) => Data :: json (value) , Err (err) => Data :: error (err . to_string () , DataFormat :: Json) , } } }
};
}
