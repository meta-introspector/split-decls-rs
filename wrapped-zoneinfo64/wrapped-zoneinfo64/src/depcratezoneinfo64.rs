// Generated macro for ZoneInfo64 (struct)
macro_rules! DepcrateZoneInfo64 {
() => {
// Module: crate
// Provides: {"ZoneInfo64"}
// Dependencies: {}
# [derive (Debug)] # [doc = " The primary type containing parsed ZoneInfo64 data"] pub struct ZoneInfo64 < 'a > { zones : Vec < TzZone < 'a > > , names : Vec < & 'a PotentialUtf16 > , rules : Vec < TzRule > , regions : Vec < Region > , }
};
}
