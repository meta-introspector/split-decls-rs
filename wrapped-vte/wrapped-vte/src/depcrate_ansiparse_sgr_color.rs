// Generated macro for parse_sgr_color (function)
macro_rules! Depcrate_ansiparse_sgr_color {
() => {
// Module: crate::ansi
// Provides: {"parse_sgr_color"}
// Dependencies: {}
# [doc = " Parse a color specifier from list of attributes."] fn parse_sgr_color (params : & mut dyn Iterator < Item = u16 >) -> Option < Color > { match params . next () { Some (2) => Some (Color :: Spec (Rgb { r : u8 :: try_from (params . next () ?) . ok () ? , g : u8 :: try_from (params . next () ?) . ok () ? , b : u8 :: try_from (params . next () ?) . ok () ? , })) , Some (5) => Some (Color :: Indexed (u8 :: try_from (params . next () ?) . ok () ?)) , _ => None , } }
};
}
