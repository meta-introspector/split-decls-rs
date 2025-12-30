// Generated macro for midi_to_freq (function)
macro_rules! Depcratemidi_to_freq {
() => {
// Module: crate
// Provides: {"midi_to_freq"}
// Dependencies: {}
# [doc = " Converts a midi note to frequency"] # [doc = ""] # [doc = " A midi note is an integer, generally in the range of 21 to 108"] pub fn midi_to_freq (note : u8) -> f32 { 27.5 * 2f32 . powf ((note as f32 - 21.0) / 12.0) }
};
}
