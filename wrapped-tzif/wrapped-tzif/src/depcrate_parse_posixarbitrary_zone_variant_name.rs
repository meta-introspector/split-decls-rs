// Generated macro for arbitrary_zone_variant_name (function)
macro_rules! Depcrate_parse_posixarbitrary_zone_variant_name {
() => {
// Module: crate::parse::posix
// Provides: {"arbitrary_zone_variant_name"}
// Dependencies: {}
# [doc = " Parses an arbitrary time-zone variant name. This name must be enclosed in angled brackets, e.g. `<name>`."] fn arbitrary_zone_variant_name < Input > () -> impl Parser < Input , Output = String > where Input : Stream < Token = u8 > , { between (byte (b'<') , byte (b'>') , many1 :: < Vec < u8 > , _ , _ > (satisfy (| byte | byte != b'>')) ,) . map (| name | String :: from_utf8_lossy (& name) . into_owned ()) }
};
}
