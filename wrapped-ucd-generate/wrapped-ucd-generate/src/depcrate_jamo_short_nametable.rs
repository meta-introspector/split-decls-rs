// Generated macro for table (function)
macro_rules! Depcrate_jamo_short_nametable {
() => {
// Module: crate::jamo_short_name
// Provides: {"table"}
// Dependencies: {}
pub fn table (dir : & Path) -> Result < Vec < (u32 , String) > > { Ok (jamo_map (dir) ? . into_iter () . collect ()) }
};
}
