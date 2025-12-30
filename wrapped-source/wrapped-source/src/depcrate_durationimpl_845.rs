// Generated macro for impl_845 (impl)
macro_rules! Depcrate_durationimpl_845 {
() => {
// Module: crate::duration
// Provides: {"impl_845"}
// Dependencies: {}
impl SourceDataProvider { # [expect (clippy :: type_complexity)] pub (crate) fn load_duration_parts_internal (& self , req : DataRequest < '_ > ,) -> Result < (u8 , u8 , & str , u8 , u8 , u8 , u8 , u8) , DataError > { let units_format_data : & cldr_serde :: units :: data :: Resource = self . cldr () ? . units () . read_and_parse (req . id . locale , "units.json") ? ; let DurationUnits { hms , hm , ms } = & units_format_data . main . value . units . duration ; let mut hm = hm . pat . as_str () ; let ([hm_hour_pad , hm_min_pad] , hm_sep) = strip_separated_padded_characters (& mut hm , ['h' , 'm']) ? ; let mut ms = ms . pat . as_str () ; let ([ms_min_pad , ms_sec_pad] , ms_sep) = strip_separated_padded_characters (& mut ms , ['m' , 's']) ? ; let mut hms = hms . pat . as_str () ; let ([hms_hour_pad , hms_min_pad , hms_sec_pad] , hms_sep) = strip_separated_padded_characters (& mut hms , ['h' , 'm' , 's']) ? ; if hm_sep != hms_sep || ms_sep != hms_sep || hm_sep != ms_sep { return Err (DataError :: custom ("Inconsistent separators in duration patterns" ,)) ; } Ok ((hm_hour_pad , hm_min_pad , hm_sep , ms_min_pad , ms_sec_pad , hms_hour_pad , hms_min_pad , hms_sec_pad ,)) } }
};
}
