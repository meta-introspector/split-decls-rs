// Generated macro for Zone (struct)
macro_rules! DepcrateZone {
() => {
// Module: crate
// Provides: {"Zone"}
// Dependencies: {}
# [doc = " Data for a given time zone"] # [derive (Clone , Copy)] pub struct Zone < 'a > { idx : u16 , resolved_idx : u16 , info : & 'a ZoneInfo64 < 'a > , }
};
}
