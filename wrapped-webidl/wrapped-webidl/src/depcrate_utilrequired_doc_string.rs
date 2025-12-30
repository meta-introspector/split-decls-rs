// Generated macro for required_doc_string (function)
macro_rules! Depcrate_utilrequired_doc_string {
() => {
// Module: crate::util
// Provides: {"required_doc_string"}
// Dependencies: {}
pub fn required_doc_string (options : & Options , features : & BTreeSet < String >) -> Option < String > { if ! options . features || features . is_empty () { return None ; } let list = features . iter () . map (| ident | format ! ("`{ident}`")) . collect :: < Vec < _ > > () . join (", ") ; Some (format ! ("\n\n*This API requires the following crate features \
         to be activated: {list}*" ,)) }
};
}
