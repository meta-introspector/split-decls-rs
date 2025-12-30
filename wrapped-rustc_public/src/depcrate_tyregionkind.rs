// Generated macro for RegionKind (enum)
macro_rules! Depcrate_tyRegionKind {
() => {
// Module: crate::ty
// Provides: {"RegionKind"}
// Dependencies: {}
# [derive (Clone , Debug , Eq , PartialEq , Hash , Serialize)] pub enum RegionKind { ReEarlyParam (EarlyParamRegion) , ReBound (DebruijnIndex , BoundRegion) , ReStatic , RePlaceholder (Placeholder < BoundRegion >) , ReErased , }
};
}
