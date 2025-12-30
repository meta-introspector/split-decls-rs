// Generated macro for fold_regions (function)
macro_rules! Depcrate_foldfold_regions {
() => {
// Module: crate::fold
// Provides: {"fold_regions"}
// Dependencies: {}
pub fn fold_regions < I : Interner , T > (cx : I , value : T , f : impl FnMut (I :: Region , ty :: DebruijnIndex) -> I :: Region ,) -> T where T : TypeFoldable < I > , { value . fold_with (& mut RegionFolder :: new (cx , f)) }
};
}
