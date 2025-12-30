// Generated macro for sm3_round2 (function)
macro_rules! Depcrate_compresssm3_round2 {
() => {
// Module: crate::compress
// Provides: {"sm3_round2"}
// Dependencies: {}
fn sm3_round2 (a : u32 , b : u32 , c : u32 , d : u32 , e : u32 , f : u32 , g : u32 , h : u32 , t : u32 , w1 : u32 , w2 : u32 ,) -> [u32 ; 8] { let ss1 = (a . rotate_left (12) . wrapping_add (e) . wrapping_add (t)) . rotate_left (7) ; let ss2 = ss1 ^ a . rotate_left (12) ; let d = d . wrapping_add (ff2 (a , b , c)) . wrapping_add (ss2) . wrapping_add (w1 ^ w2) ; let h = h . wrapping_add (gg2 (e , f , g)) . wrapping_add (ss1) . wrapping_add (w1) ; let b = b . rotate_left (9) ; let f = f . rotate_left (19) ; let h = p0 (h) ; [a , b , c , d , e , f , g , h] }
};
}
