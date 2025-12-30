// Generated macro for impl_54 (impl)
macro_rules! Depcrateimpl_54 {
() => {
// Module: crate
// Provides: {"impl_54"}
// Dependencies: {}
impl < 'a > Zone < 'a > { # [doc = " Decomposes this [`Zone`] into its raw parts, consisting"] # [doc = " of some state stored in a `u16`, and the associated [`ZoneInfo64`]."] # [doc = ""] # [doc = " See [`Self::from_raw_parts`] for the inverse operation."] pub fn into_raw_parts (self) -> (u16 , & 'a ZoneInfo64 < 'a >) { (self . idx , self . info) } # [doc = " Recreates the [`Zone`] from raw parts."] # [doc = ""] # [doc = " Returns garbage if `parts` was not obtained from [`Self::into_raw_parts`]."] pub fn from_raw_parts (parts : (u16 , & 'a ZoneInfo64 < 'a >)) -> Self { let (idx , info) = parts ; let idx = core :: cmp :: min (info . zones . len () - 1 , idx as usize) ; # [expect (clippy :: indexing_slicing)] let resolved_idx = if let TzZone :: Int (i) = info . zones [idx] { i as u16 } else { idx as u16 } ; Self { idx : idx as u16 , resolved_idx , info , } } }
};
}
