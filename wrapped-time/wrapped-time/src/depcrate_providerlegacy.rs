// Generated macro for legacy (module)
macro_rules! Depcrate_providerlegacy {
() => {
// Module: crate::provider
// Provides: {"legacy"}
// Dependencies: {}
# [cfg (feature = "alloc")] pub (crate) mod legacy { use super :: * ; use zerovec :: ZeroMap2d ; icu_provider :: data_marker ! (# [doc = " The default mapping between period and offsets. The second level key is a wall-clock time encoded as"] # [doc = " [`ZoneNameTimestamp`]. It represents when the offsets started to be used."] TimezoneVariantsOffsetsV1 , "timezone/variants/offsets/v1" , ZeroMap2d <'static , TimeZone , ZoneNameTimestamp , VariantOffsets >, is_singleton = true) ; }
};
}
