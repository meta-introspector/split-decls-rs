// Generated macro for Entry (struct)
macro_rules! Depcrate_fse_fse_decoderEntry {
() => {
// Module: crate::fse::fse_decoder
// Provides: {"Entry"}
// Dependencies: {}
# [doc = " A single entry in an FSE table."] # [derive (Copy , Clone , Debug)] pub struct Entry { # [doc = " This value is used as an offset value, and it is added"] # [doc = " to a value read from the stream to determine the next state value."] pub base_line : u32 , # [doc = " How many bits should be read from the stream when decoding this entry."] pub num_bits : u8 , # [doc = " The byte that should be put in the decode output when encountering this state."] pub symbol : u8 , }
};
}
