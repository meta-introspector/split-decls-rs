// Generated macro for impl_67 (impl)
macro_rules! Depcrate_hypertreeimpl_67 {
() => {
// Module: crate::hypertree
// Provides: {"impl_67"}
// Dependencies: {}
impl < P : HypertreeParams > TryFrom < & [u8] > for HypertreeSig < P > { type Error = () ; fn try_from (value : & [u8]) -> Result < Self , Self :: Error > { if value . len () != Self :: SIZE { return Err (()) ; } let sig = value . chunks (XmssSig :: < P > :: SIZE) . map (| c | XmssSig :: try_from (c) . unwrap ()) . collect () ; Ok (HypertreeSig (sig)) } }
};
}
