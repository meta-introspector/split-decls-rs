// Generated macro for octal_into (function)
macro_rules! Depcrate_headeroctal_into {
() => {
// Module: crate::header
// Provides: {"octal_into"}
// Dependencies: {}
fn octal_into < T : fmt :: Octal > (dst : & mut [u8] , val : T) { let o = format ! ("{:o}" , val) ; let value = once (b'\0') . chain (o . bytes () . rev () . chain (repeat (b'0'))) ; for (slot , value) in dst . iter_mut () . rev () . zip (value) { * slot = value ; } }
};
}
