// Generated macro for sha1rnds4c (function)
macro_rules! Depcratesha1rnds4c {
() => {
// Module: crate
// Provides: {"sha1rnds4c"}
// Dependencies: {}
# [doc = " Not an intrinsic, but helps emulate `llvm.x86.sha1rnds4` intrinsic."] fn sha1rnds4c (abcd : u32x4 , msg : u32x4) -> u32x4 { let u32x4 (mut a , mut b , mut c , mut d) = abcd ; let u32x4 (t , u , v , w) = msg ; let mut e = 0u32 ; macro_rules ! bool3ary_202 { ($ a : expr , $ b : expr , $ c : expr) => { ($ c ^ ($ a & ($ b ^ $ c))) } ; } e = e . wrapping_add (a . rotate_left (5)) . wrapping_add (bool3ary_202 ! (b , c , d)) . wrapping_add (t) ; b = b . rotate_left (30) ; d = d . wrapping_add (e . rotate_left (5)) . wrapping_add (bool3ary_202 ! (a , b , c)) . wrapping_add (u) ; a = a . rotate_left (30) ; c = c . wrapping_add (d . rotate_left (5)) . wrapping_add (bool3ary_202 ! (e , a , b)) . wrapping_add (v) ; e = e . rotate_left (30) ; b = b . wrapping_add (c . rotate_left (5)) . wrapping_add (bool3ary_202 ! (d , e , a)) . wrapping_add (w) ; d = d . rotate_left (30) ; u32x4 (b , c , d , e) }
};
}
