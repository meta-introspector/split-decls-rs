// Generated macro for to_utf8 (function)
macro_rules! Depcrate__private_rustdocto_utf8 {
() => {
// Module: crate::_private::rustdoc
// Provides: {"to_utf8"}
// Dependencies: {}
const fn to_utf8 (bytes : & [u8]) -> & str { match core :: str :: from_utf8 (bytes) { Ok (x) => x , Err (_) => panic ! ("Invalid UTF-8") , } }
};
}
