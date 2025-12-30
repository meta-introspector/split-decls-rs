// Generated macro for Parser (struct)
macro_rules! Depcrate_parserParser {
() => {
// Module: crate::parser
// Provides: {"Parser"}
// Dependencies: {}
# [doc = " A YAML parser."] # [derive (Debug)] pub struct Parser < T > { scanner : Scanner < T > , states : Vec < State > , state : State , token : Option < Token > , current : Option < (Event , Marker) > , anchors : HashMap < String , usize > , anchor_id : usize , # [doc = " The tag directives (`%TAG`) the parser has encountered."] # [doc = ""] # [doc = " Key is the handle, and value is the prefix."] tags : HashMap < String , String > , # [doc = " Make tags global across all documents."] keep_tags : bool , }
};
}
