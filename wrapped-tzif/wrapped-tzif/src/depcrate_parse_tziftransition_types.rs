// Generated macro for transition_types (function)
macro_rules! Depcrate_parse_tziftransition_types {
() => {
// Module: crate::parse::tzif
// Provides: {"transition_types"}
// Dependencies: {}
# [doc = " A series of one-byte unsigned integers specifying"] # [doc = " the type of local time of the corresponding transition time."] # [doc = ""] # [doc = " These values serve as zero-based indices into the array of local"] # [doc = " time type records. The number of type indices is specified by the"] # [doc = " `timecnt` field in the header. Each type index MUST be in the"] # [doc = " range `[0, typecnt - 1]`"] fn transition_types < Input > (timecnt : usize , typecnt : usize ,) -> impl Parser < Input , Output = Vec < usize > > where Input : Stream < Token = u8 > , Input :: Error : ParseError < Input :: Token , Input :: Range , Input :: Position > , { count_min_max (timecnt , timecnt , any () . map (| byte | byte as usize)) . then (move | types : Vec < usize > | { ensure (types , | types | types . iter () . all (| & t | t < typecnt) , "all transition types should be in range [0, typecnt - 1]" ,) } ,) }
};
}
