// Generated macro for detect_url (function)
macro_rules! Depcrate_stringdetect_url {
() => {
// Module: crate::string
// Provides: {"detect_url"}
// Dependencies: {}
# [doc = " Returns the index to the end of the URL if the split at index of the given string includes a"] # [doc = " URL or alike. Otherwise, returns `None`."] fn detect_url (s : & [& str] , index : usize) -> Option < usize > { let start = match s [..= index] . iter () . rposition (| g | is_whitespace (g)) { Some (pos) => pos + 1 , None => 0 , } ; if s . len () < start + 8 { return None ; } let split = s [start ..] . concat () ; if split . contains ("https://") || split . contains ("http://") || split . contains ("ftp://") || split . contains ("file://") { match s [index ..] . iter () . position (| g | is_whitespace (g)) { Some (pos) => Some (index + pos - 1) , None => Some (s . len () - 1) , } } else { None } }
};
}
