// Generated macro for local_time_type_record (function)
macro_rules! Depcrate_parse_tziflocal_time_type_record {
() => {
// Module: crate::parse::tzif
// Provides: {"local_time_type_record"}
// Dependencies: {}
# [doc = " A series of six-byte records specifying a"] # [doc = " local time type. Each record has the following"] # [doc = " format (the lengths of multi-byte fields are shown in"] # [doc = " parentheses):"] # [doc = ""] # [doc = " > ```text"] # [doc = " > +---------------+---+---+"] # [doc = " > |  utoff (4)    |dst|idx|"] # [doc = " > +---------------+---+---+"] # [doc = " > ```"] fn local_time_type_record < Input > (charcnt : usize) -> impl Parser < Input , Output = LocalTimeTypeRecord > where Input : Stream < Token = u8 > , Input :: Error : ParseError < Input :: Token , Input :: Range , Input :: Position > , { combine :: struct_parser ! { LocalTimeTypeRecord { utoff : utoff () , is_dst : is_dst () , idx : idx (charcnt) , } } }
};
}
