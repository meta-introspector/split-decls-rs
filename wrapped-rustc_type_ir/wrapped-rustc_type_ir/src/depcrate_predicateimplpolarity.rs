// Generated macro for ImplPolarity (enum)
macro_rules! Depcrate_predicateImplPolarity {
() => {
// Module: crate::predicate
// Provides: {"ImplPolarity"}
// Dependencies: {}
# [derive (Clone , Copy , PartialEq , Eq , PartialOrd , Ord , Hash , Debug)] # [cfg_attr (feature = "nightly" , derive (Decodable_NoContext , Encodable_NoContext , HashStable_NoContext))] pub enum ImplPolarity { # [doc = " `impl Trait for Type`"] Positive , # [doc = " `impl !Trait for Type`"] Negative , # [doc = " `#[rustc_reservation_impl] impl Trait for Type`"] # [doc = ""] # [doc = " This is a \"stability hack\", not a real Rust feature."] # [doc = " See #64631 for details."] Reservation , }
};
}
