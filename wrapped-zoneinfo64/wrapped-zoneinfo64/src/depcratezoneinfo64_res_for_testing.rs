// Generated macro for ZONEINFO64_RES_FOR_TESTING (const)
macro_rules! DepcrateZONEINFO64_RES_FOR_TESTING {
() => {
// Module: crate
// Provides: {"ZONEINFO64_RES_FOR_TESTING"}
// Dependencies: {}
# [doc = " A bundled zoneinfo64.res that can be used for testing. No guarantee is made"] # [doc = " as to the version in use; though we will try to keep it up to date."] pub const ZONEINFO64_RES_FOR_TESTING : & [u32] = resb :: include_bytes_as_u32 ! ("./data/zoneinfo64.res") ;
};
}
