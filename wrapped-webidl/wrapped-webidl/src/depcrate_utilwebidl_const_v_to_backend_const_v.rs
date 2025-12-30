// Generated macro for webidl_const_v_to_backend_const_v (function)
macro_rules! Depcrate_utilwebidl_const_v_to_backend_const_v {
() => {
// Module: crate::util
// Provides: {"webidl_const_v_to_backend_const_v"}
// Dependencies: {}
# [doc = " Map a webidl const value to the correct wasm-bindgen const value"] pub fn webidl_const_v_to_backend_const_v (v : & ConstValueLit) -> ConstValue { match * v { ConstValueLit :: Boolean (b) => ConstValue :: Boolean (b . 0) , ConstValueLit :: Float (FloatLit :: NegInfinity (_)) => ConstValue :: Float (f64 :: NEG_INFINITY) , ConstValueLit :: Float (FloatLit :: Infinity (_)) => ConstValue :: Float (f64 :: INFINITY) , ConstValueLit :: Float (FloatLit :: NaN (_)) => ConstValue :: Float (f64 :: NAN) , ConstValueLit :: Float (FloatLit :: Value (s)) => ConstValue :: Float (s . 0 . parse () . unwrap ()) , ConstValueLit :: Integer (lit) => { let mklit = | orig_text : & str , base : u32 , offset : usize | { let (negative , text) = if let Some (text) = orig_text . strip_prefix ('-') { (true , text) } else { (false , orig_text) } ; if text == "0" { return ConstValue :: SignedInteger (0) ; } let text = & text [offset ..] ; let n = u64 :: from_str_radix (text , base) . unwrap_or_else (| _ | panic ! ("literal too big: {orig_text}")) ; if negative { let n = if n > (i64 :: MIN as u64) . wrapping_neg () { panic ! ("literal too big: {orig_text}") } else { n . wrapping_neg () as i64 } ; ConstValue :: SignedInteger (n) } else { ConstValue :: UnsignedInteger (n) } } ; match lit { IntegerLit :: Hex (h) => mklit (h . 0 , 16 , 2) , IntegerLit :: Oct (h) => mklit (h . 0 , 8 , 1) , IntegerLit :: Dec (h) => mklit (h . 0 , 10 , 0) , } } ConstValueLit :: Null (_) => unimplemented ! () , } }
};
}
