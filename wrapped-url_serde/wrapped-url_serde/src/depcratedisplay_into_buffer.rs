// Generated macro for display_into_buffer (function)
macro_rules! Depcratedisplay_into_buffer {
() => {
// Module: crate
// Provides: {"display_into_buffer"}
// Dependencies: {}
# [doc = " Like .to_string(), but doesn’t allocate memory for a `String`."] # [doc = ""] # [doc = " Panics if `buffer` is too small."] fn display_into_buffer < 'a , T : fmt :: Display > (value : & T , buffer : & 'a mut [u8]) -> & 'a str { let remaining_len ; { let mut remaining = & mut * buffer ; write ! (remaining , "{}" , value) . unwrap () ; remaining_len = remaining . len () } let written_len = buffer . len () - remaining_len ; let written = & buffer [.. written_len] ; # [allow (unsafe_code)] unsafe { str :: from_utf8_unchecked (written) } }
};
}
