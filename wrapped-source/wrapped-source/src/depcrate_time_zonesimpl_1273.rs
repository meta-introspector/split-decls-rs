// Generated macro for impl_1273 (impl)
macro_rules! Depcrate_time_zonesimpl_1273 {
() => {
// Module: crate::time_zones
// Provides: {"impl_1273"}
// Dependencies: {}
impl std :: fmt :: Debug for Transition { fn fmt (& self , f : & mut std :: fmt :: Formatter < '_ >) -> std :: fmt :: Result { write ! (f , "{:04}-{:02}-{:02} {:02}:{:02}" , self . transition . date . era_year () . year , self . transition . date . month () . ordinal , self . transition . date . day_of_month () . 0 , self . transition . time . hour . number () , self . transition . time . minute . number ()) ? ; write ! (f , " - {:?} - {}+{}" , self . name , self . utc_offset , self . dst_offset_relative) ? ; if self . rearguard_agrees == Some (false) { write ! (f , " !rearguard") ? ; } if self . vanguard_agrees == Some (false) { write ! (f , " !vanguard") ? ; } Ok (()) } }
};
}
