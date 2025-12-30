// Generated macro for escaped_transform (function)
macro_rules! Depcrate_asciiescaped_transform {
() => {
// Module: crate::ascii
// Provides: {"escaped_transform"}
// Dependencies: {}
# [doc = " Deprecated, replaed with [`escaped`]"] # [inline (always)] # [deprecated (since = "7.0.0" , note = "replaced with `escaped`")] pub fn escaped_transform < Input , Error , Normal , NormalOutput , Escape , EscapeOutput , Output > (normal : Normal , control_char : char , escape : Escape ,) -> impl Parser < Input , Output , Error > where Input : StreamIsPartial + Stream + Compare < char > , Normal : Parser < Input , NormalOutput , Error > , Escape : Parser < Input , EscapeOutput , Error > , Output : crate :: stream :: Accumulate < NormalOutput > , Output : crate :: stream :: Accumulate < EscapeOutput > , Error : ParserError < Input > , { escaped (normal , control_char , escape) }
};
}
