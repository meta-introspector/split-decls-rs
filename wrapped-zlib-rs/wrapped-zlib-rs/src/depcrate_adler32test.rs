// Generated macro for test (module)
macro_rules! Depcrate_adler32test {
() => {
// Module: crate::adler32
// Provides: {"test"}
// Dependencies: {}
# [cfg (test)] mod test { use super :: * ; # [test] fn naive_is_fancy_small_inputs () { for i in 0 .. 128 { let v = (0u8 .. i) . collect :: < Vec < _ > > () ; assert_eq ! (naive_adler32 (1 , & v) , generic :: adler32_rust (1 , & v)) ; } } # [test] fn test_adler32_combine () { :: quickcheck :: quickcheck (test as fn (_) -> _) ; fn test (data : Vec < u8 >) -> bool { let Some (buf_len) = data . first () . copied () else { return true ; } ; let buf_size = Ord :: max (buf_len , 1) as usize ; let mut adler1 = 1 ; let mut adler2 = 1 ; for chunk in data . chunks (buf_size) { adler1 = adler32 (adler1 , chunk) ; } adler2 = adler32 (adler2 , & data) ; assert_eq ! (adler1 , adler2) ; let combine1 = adler32_combine (adler1 , adler2 , data . len () as _) ; let combine2 = adler32_combine (adler1 , adler1 , data . len () as _) ; assert_eq ! (combine1 , combine2) ; true } } }
};
}
