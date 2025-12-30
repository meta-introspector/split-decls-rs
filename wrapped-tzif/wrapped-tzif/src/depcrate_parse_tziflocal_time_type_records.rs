// Generated macro for local_time_type_records (function)
macro_rules! Depcrate_parse_tziflocal_time_type_records {
() => {
// Module: crate::parse::tzif
// Provides: {"local_time_type_records"}
// Dependencies: {}
# [doc = " A series of local time type records."] # [doc = " The number of records is specified by the \"typecnt\" field in the header."] fn local_time_type_records < Input > (typecnt : usize , charcnt : usize ,) -> impl Parser < Input , Output = Vec < LocalTimeTypeRecord > > where Input : Stream < Token = u8 > , Input :: Error : ParseError < Input :: Token , Input :: Range , Input :: Position > , { count_min_max (typecnt , typecnt , local_time_type_record (charcnt)) }
};
}
