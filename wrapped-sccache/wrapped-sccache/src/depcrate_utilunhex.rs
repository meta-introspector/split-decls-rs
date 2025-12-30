// Generated macro for unhex (function)
macro_rules! Depcrate_utilunhex {
() => {
// Module: crate::util
// Provides: {"unhex"}
// Dependencies: {}
fn unhex (b : u8) -> std :: io :: Result < u8 > { match b { b'0' ..= b'9' => Ok (b - b'0') , b'a' ..= b'f' => Ok (b - b'a' + 10) , b'A' ..= b'F' => Ok (b - b'A' + 10) , _ => Err (std :: io :: Error :: new (std :: io :: ErrorKind :: InvalidInput , "invalid hex digit" ,)) , } }
};
}
