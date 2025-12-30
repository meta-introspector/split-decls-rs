// Generated macro for backslash_x (function)
macro_rules! Depcrate_helpers_stringbackslash_x {
() => {
// Module: crate::helpers::string
// Provides: {"backslash_x"}
// Dependencies: {}
fn backslash_x < S > (s : & S) -> (u8 , & S) where S : Index < RangeFrom < usize > , Output = S > + AsRef < [u8] > + ? Sized , { let mut ch = 0 ; let b0 = byte (s , 0) ; let b1 = byte (s , 1) ; ch += 0x10 * (b0 - b'0') ; ch += match b1 { b'0' ..= b'9' => b1 - b'0' , b'a' ..= b'f' => 10 + (b1 - b'a') , b'A' ..= b'F' => 10 + (b1 - b'A') , _ => bug ! ("invalid hex escape") , } ; (ch , & s [2 ..]) }
};
}
