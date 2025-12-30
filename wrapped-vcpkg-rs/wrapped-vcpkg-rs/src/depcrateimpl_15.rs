// Generated macro for impl_15 (impl)
macro_rules! Depcrateimpl_15 {
() => {
// Module: crate
// Provides: {"impl_15"}
// Dependencies: {}
impl < S : AsRef < str > > From < S > for TargetTriplet { fn from (triplet : S) -> TargetTriplet { let triplet = triplet . as_ref () ; if triplet . contains ("windows") { TargetTriplet { triplet : triplet . into () , is_static : triplet . contains ("-static") , lib_suffix : "lib" . into () , strip_lib_prefix : false , } } else { TargetTriplet { triplet : triplet . into () , is_static : true , lib_suffix : "a" . into () , strip_lib_prefix : true , } } } }
};
}
