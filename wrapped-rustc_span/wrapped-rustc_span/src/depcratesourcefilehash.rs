// Generated macro for SourceFileHash (struct)
macro_rules! DepcrateSourceFileHash {
() => {
// Module: crate
// Provides: {"SourceFileHash"}
// Dependencies: {}
# [doc = " The hash of the on-disk source file used for debug info and cargo freshness checks."] # [derive (Copy , Clone , PartialEq , Eq , Debug , Hash)] # [derive (HashStable_Generic , Encodable , Decodable)] pub struct SourceFileHash { pub kind : SourceFileHashAlgorithm , value : [u8 ; 32] , }
};
}
