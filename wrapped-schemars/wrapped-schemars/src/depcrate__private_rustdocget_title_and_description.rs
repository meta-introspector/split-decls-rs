// Generated macro for get_title_and_description (function)
macro_rules! Depcrate__private_rustdocget_title_and_description {
() => {
// Module: crate::_private::rustdoc
// Provides: {"get_title_and_description"}
// Dependencies: {}
# [must_use] pub const fn get_title_and_description (doc : & str) -> (& str , & str) { let doc_bytes = trim_ascii (doc . as_bytes ()) ; if ! doc_bytes . is_empty () && doc_bytes [0] == b'#' { let title_end_index = match strchr (doc_bytes , b'\n') { Some (i) => i , None => doc_bytes . len () , } ; let title = trim_ascii (trim_start (subslice (doc_bytes , 0 , title_end_index) , b'#')) ; let description = trim_ascii (subslice (doc_bytes , title_end_index , doc_bytes . len ())) ; (to_utf8 (title) , to_utf8 (description)) } else { ("" , to_utf8 (doc_bytes)) } }
};
}
