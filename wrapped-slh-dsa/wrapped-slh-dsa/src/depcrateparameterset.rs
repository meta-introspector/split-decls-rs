// Generated macro for ParameterSet (trait)
macro_rules! DepcrateParameterSet {
() => {
// Module: crate
// Provides: {"ParameterSet"}
// Dependencies: {}
# [doc = " Specific parameters for each of the 12 FIPS parameter sets"] # [allow (private_bounds)] pub trait ParameterSet : ForsParams + SigningKeyLen + VerifyingKeyLen + SignatureLen + PartialEq + Eq { # [doc = " Human-readable name for parameter set, matching the FIPS-205 designations"] const NAME : & 'static str ; # [doc = " Associated OID with the Parameter"] const ALGORITHM_OID : pkcs8 :: ObjectIdentifier ; }
};
}
