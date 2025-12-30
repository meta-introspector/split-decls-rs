// Generated macro for macro_2 (macro)
macro_rules! Depcratemacro_2 {
() => {
// Module: crate
// Provides: {"macro_2"}
// Dependencies: {}
bitflags ! { # [derive (Debug)] # [doc = " Example Flags"] pub struct Flags : u32 { # [doc = " A"] const A = 0b0000_0001 ; # [doc = " B"] const B = 0b0000_0010 ; # [doc = " C"] const C = 0b0000_0100 ; # [doc = " ABC"] const ABC = Flags :: A . bits () | Flags :: B . bits () | Flags :: C . bits () ; const _ = ! 0 ; } }
};
}
