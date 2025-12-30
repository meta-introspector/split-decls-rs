// Generated macro for BYTE_STR_SENTINEL (const)
macro_rules! Depcrate_serializeBYTE_STR_SENTINEL {
() => {
// Module: crate::serialize
// Provides: {"BYTE_STR_SENTINEL"}
// Dependencies: {}
# [doc = " For byte strings there are no bytes that cannot occur. Just use this value"] # [doc = " as a best-effort sentinel. There is no validation skipped so the potential"] # [doc = " for badness is lower than in the `STR_SENTINEL` case."] const BYTE_STR_SENTINEL : u8 = 0xC2 ;
};
}
