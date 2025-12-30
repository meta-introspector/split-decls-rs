// Generated macro for State (struct)
macro_rules! Depcrate_fse_fse_encoderState {
() => {
// Module: crate::fse::fse_encoder
// Provides: {"State"}
// Dependencies: {}
# [derive (Debug , Clone)] pub (crate) struct State { # [doc = " How many bits the range of this state needs to be encoded as"] pub (crate) num_bits : u8 , # [doc = " The first index targeted by this state"] pub (crate) baseline : usize , # [doc = " The last index targeted by this state (baseline + the maximum number with numbits bits allows)"] pub (crate) last_index : usize , # [doc = " Index of this state in the decoding table"] pub (crate) index : usize , }
};
}
