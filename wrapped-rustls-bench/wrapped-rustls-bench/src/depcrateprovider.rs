// Generated macro for Provider (enum)
macro_rules! DepcrateProvider {
() => {
// Module: crate
// Provides: {"Provider"}
// Dependencies: {}
# [derive (Copy , Clone , Debug , PartialEq , ValueEnum)] enum Provider { # [cfg (feature = "aws-lc-rs")] AwsLcRs , # [cfg (all (feature = "aws-lc-rs" , feature = "fips"))] AwsLcRsFips , # [cfg (feature = "graviola")] Graviola , # [cfg (feature = "ring")] Ring , # [value (skip)] _None , }
};
}
