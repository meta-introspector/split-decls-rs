// Generated macro for starts_with_windows_drive_letter_segment (function)
macro_rules! Depcrate_parserstarts_with_windows_drive_letter_segment {
() => {
// Module: crate::parser
// Provides: {"starts_with_windows_drive_letter_segment"}
// Dependencies: {}
# [doc = " https://url.spec.whatwg.org/#start-with-a-windows-drive-letter"] fn starts_with_windows_drive_letter_segment (input : & Input < '_ >) -> bool { let mut input = input . clone () ; match (input . next () , input . next () , input . next ()) { (Some (a) , Some (b) , Some (c)) if ascii_alpha (a) && matches ! (b , ':' | '|') && matches ! (c , '/' | '\\' | '?' | '#') => { true } (Some (a) , Some (b) , None) if ascii_alpha (a) && matches ! (b , ':' | '|') => true , _ => false , } }
};
}
