// Generated macro for attr_impl (module)
macro_rules! Depcrate_callconvattr_impl {
() => {
// Module: crate::callconv
// Provides: {"attr_impl"}
// Dependencies: {}
# [allow (non_upper_case_globals)] # [allow (unused)] mod attr_impl { use rustc_macros :: HashStable_Generic ; # [derive (Clone , Copy , Default , Hash , PartialEq , Eq , HashStable_Generic)] pub struct ArgAttribute (u8) ; bitflags :: bitflags ! { impl ArgAttribute : u8 { const NoAlias = 1 << 1 ; const CapturesAddress = 1 << 2 ; const NonNull = 1 << 3 ; const ReadOnly = 1 << 4 ; const InReg = 1 << 5 ; const NoUndef = 1 << 6 ; const CapturesReadOnly = 1 << 7 ; } } rustc_data_structures :: external_bitflags_debug ! { ArgAttribute } }
};
}
