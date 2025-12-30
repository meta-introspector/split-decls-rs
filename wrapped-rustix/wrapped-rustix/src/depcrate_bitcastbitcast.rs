// Generated macro for bitcast (macro)
macro_rules! Depcrate_bitcastbitcast {
() => {
// Module: crate::bitcast
// Provides: {"bitcast"}
// Dependencies: {}
macro_rules ! bitcast { ($ x : expr) => { { if false { let _ = !$ x ; let _ = $ x as u8 ; 0 } else if false { # [allow (unsafe_code , unused_unsafe , clippy :: useless_transmute , clippy :: missing_transmute_annotations)] unsafe { :: core :: mem :: transmute ($ x) } } else { $ x as _ } } } ; }
};
}
