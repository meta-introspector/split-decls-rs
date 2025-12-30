// Generated macro for set_href (function)
macro_rules! Depcrate_quirksset_href {
() => {
// Module: crate::quirks
// Provides: {"set_href"}
// Dependencies: {}
# [doc = " Setter for <https://url.spec.whatwg.org/#dom-url-href>"] pub fn set_href (url : & mut Url , value : & str) -> Result < () , ParseError > { * url = Url :: parse (value) ? ; Ok (()) }
};
}
