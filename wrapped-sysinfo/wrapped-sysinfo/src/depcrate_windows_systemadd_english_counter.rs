// Generated macro for add_english_counter (function)
macro_rules! Depcrate_windows_systemadd_english_counter {
() => {
// Module: crate::windows::system
// Provides: {"add_english_counter"}
// Dependencies: {}
fn add_english_counter (s : String , query : & mut super :: cpu :: Query , keys : & mut Option < KeyHandler > , counter_name : String ,) { let mut full = s . encode_utf16 () . collect :: < Vec < _ > > () ; full . push (0) ; if query . add_english_counter (& counter_name , full) { * keys = Some (KeyHandler :: new (counter_name)) ; } }
};
}
