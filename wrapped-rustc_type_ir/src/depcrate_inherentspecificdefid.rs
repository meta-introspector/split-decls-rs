// Generated macro for SpecificDefId (trait)
macro_rules! Depcrate_inherentSpecificDefId {
() => {
// Module: crate::inherent
// Provides: {"SpecificDefId"}
// Dependencies: {}
pub trait SpecificDefId < I : Interner > : DefId < I > + Into < I :: DefId > + TryFrom < I :: DefId , Error : std :: fmt :: Debug > { }
};
}
