// Generated macro for TimeZoneVariant (enum)
macro_rules! Depcrate_providerTimeZoneVariant {
() => {
// Module: crate::provider
// Provides: {"TimeZoneVariant"}
// Dependencies: {}
# [doc = " A time zone variant used to identify a display name in CLDR."] # [derive (Copy , Clone , Debug , Eq , PartialEq , Ord , PartialOrd , Hash)] # [zerovec :: make_ule (TimeZoneVariantULE)] # [repr (u8)] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize , databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_time :: provider))] # [cfg_attr (feature = "serde" , derive (serde :: Deserialize))] # [cfg_attr (not (feature = "alloc") , zerovec :: skip_derive (ZeroMapKV))] # [non_exhaustive] pub enum TimeZoneVariant { # [doc = " The variant corresponding to `\"standard\"` in CLDR."] # [doc = ""] # [doc = " The semantics vary from time zone to time zone. The time zone display"] # [doc = " name of this variant may or may not be called \"Standard Time\"."] Standard = 0 , # [doc = " The variant corresponding to `\"daylight\"` in CLDR."] # [doc = ""] # [doc = " The semantics vary from time zone to time zone. The time zone display"] # [doc = " name of this variant may or may not be called \"Daylight Time\"."] Daylight = 1 , }
};
}
