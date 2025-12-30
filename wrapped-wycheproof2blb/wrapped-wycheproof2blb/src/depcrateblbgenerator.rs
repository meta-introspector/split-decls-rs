// Generated macro for BlbGenerator (type)
macro_rules! DepcrateBlbGenerator {
() => {
// Module: crate
// Provides: {"BlbGenerator"}
// Dependencies: {}
# [doc = " Generator function which takes input parameters:"] # [doc = " - contents of Wycheproof test data file"] # [doc = " - algorithm name"] # [doc = " - key size (in bits) to include"] # [doc = "   and returns the raw contents, together  with a list of test identifiers (one per entry)."] type BlbGenerator = fn (& [u8] , & str , u32) -> Vec < TestInfo > ;
};
}
