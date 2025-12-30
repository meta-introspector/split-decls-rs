// Generated macro for TimezonePeriods (struct)
macro_rules! Depcrate_providerTimezonePeriods {
() => {
// Module: crate::provider
// Provides: {"TimezonePeriods"}
// Dependencies: {}
# [doc = " Data struct for the [`TimezonePeriodsV1`] marker."] # [derive (PartialEq , Debug , Clone , yoke :: Yokeable , zerofrom :: ZeroFrom)] # [cfg_attr (feature = "datagen" , derive (databake :: Bake))] # [cfg_attr (feature = "datagen" , databake (path = icu_time :: provider))] pub struct TimezonePeriods < 'a > { # [doc = " Index of `TimeZone`s into `list`."] pub index : ZeroTrieSimpleAscii < ZeroVec < 'a , u8 > > , # [doc = " Each entry contains at least one period, which implicitly starts at the UNIX epoch."] # [doc = " This is stored in the first tuple element."] # [doc = ""] # [doc = " If more periods are required the second tuple element contains them, along with their"] # [doc = " starting timestamp. These entries are ordered chronologically."] # [doc = ""] # [doc = " The values (`(u8, Option<MetazoneId>)`) are an index into the `offsets` list for the offset"] # [doc = " that the zone observes in that period, and optionally whether it is part of a metazone."] pub list : VarZeroVec < 'a , VarTupleULE < (u8 , NichedOption < MetazoneId , 1 >) , ZeroSlice < (Timestamp24 , u8 , NichedOption < MetazoneId , 1 >) > , > , > , # [doc = " The deduplicated list of offsets."] # [doc = ""] # [doc = " There are currently 99 unique VariantOffsetsWithMetazoneMembershipKind, so storing the index as a u8 is plenty enough."] pub offsets : ZeroVec < 'a , VariantOffsetsWithMetazoneMembershipKind > , }
};
}
