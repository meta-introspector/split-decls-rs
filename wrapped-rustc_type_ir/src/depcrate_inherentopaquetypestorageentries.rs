// Generated macro for OpaqueTypeStorageEntries (trait)
macro_rules! Depcrate_inherentOpaqueTypeStorageEntries {
() => {
// Module: crate::inherent
// Provides: {"OpaqueTypeStorageEntries"}
// Dependencies: {}
pub trait OpaqueTypeStorageEntries : Debug + Copy + Default { # [doc = " Whether the number of opaques has changed in a way that necessitates"] # [doc = " reevaluating a goal. For now, this is only when the number of non-duplicated"] # [doc = " entries changed."] fn needs_reevaluation (self , canonicalized : usize) -> bool ; }
};
}
