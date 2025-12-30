// Generated macro for TimeZonePeriodsSerde (struct)
macro_rules! Depcrate_providerTimeZonePeriodsSerde {
() => {
// Module: crate::provider
// Provides: {"TimeZonePeriodsSerde"}
// Dependencies: {}
# [cfg (feature = "serde")] # [derive (serde :: Deserialize)] # [cfg_attr (feature = "datagen" , derive (serde :: Serialize))] struct TimeZonePeriodsSerde < 'a > { # [serde (borrow)] pub index : ZeroTrieSimpleAscii < ZeroVec < 'a , u8 > > , # [serde (borrow)] pub list : VarZeroVec < 'a , VarTupleULE < (u8 , NichedOption < MetazoneId , 1 >) , ZeroSlice < (Timestamp24 , u8 , NichedOption < MetazoneId , 1 >) > , > , > , pub offsets : ZeroVec < 'a , VariantOffsetsWithMetazoneMembershipKind > , }
};
}
