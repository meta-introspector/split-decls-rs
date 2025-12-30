// Generated macro for zone_variant_name (function)
macro_rules! Depcrate_parse_posixzone_variant_name {
() => {
// Module: crate::parse::posix
// Provides: {"zone_variant_name"}
// Dependencies: {}
# [doc = " The string specifies the name of the time zone variant. It must be three or more characters"] # [doc = " long and must not contain a leading colon."] # [doc = ""] # [doc = " The name must not contain embedded digits, commas, plus nor minus signs unless it is specified"] # [doc = " as an arbitrary name surrounded by angled brackets, e.g. `<name>`"] # [doc = ""] # [doc = " The angled brackets will not show up in the parsed name."] fn zone_variant_name < Input > () -> impl Parser < Input , Output = String > where Input : Stream < Token = u8 > , Input :: Error : ParseError < Input :: Token , Input :: Range , Input :: Position > , { choice ((arbitrary_zone_variant_name () , alphabetic_zone_variant_name () ,)) . then (| name | { ensure (name , | name | name . len () >= 3 , "zone variant name should be 3 or more characters long" ,) }) . then (| name | { ensure (name , | name | name . as_bytes () [0] != b':' , "zone variant name should never start with a leading colon" ,) }) }
};
}
