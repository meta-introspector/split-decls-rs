// Generated macro for Url (struct)
macro_rules! DepcrateUrl {
() => {
// Module: crate
// Provides: {"Url"}
// Dependencies: {}
# [doc = " A parsed URL record."] # [derive (Clone)] pub struct Url { # [doc = " Syntax in pseudo-BNF:"] # [doc = ""] # [doc = "   url = scheme \":\" [ hierarchical | non-hierarchical ] [ \"?\" query ]? [ \"#\" fragment ]?"] # [doc = "   non-hierarchical = non-hierarchical-path"] # [doc = "   non-hierarchical-path = /* Does not start with \"/\" */"] # [doc = "   hierarchical = authority? hierarchical-path"] # [doc = "   authority = \"//\" userinfo? host [ \":\" port ]?"] # [doc = "   userinfo = username [ \":\" password ]? \"@\""] # [doc = "   hierarchical-path = [ \"/\" path-segment ]+"] serialization : String , scheme_end : u32 , username_end : u32 , host_start : u32 , host_end : u32 , host : HostInternal , port : Option < u16 > , path_start : u32 , query_start : Option < u32 > , fragment_start : Option < u32 > , }
};
}
