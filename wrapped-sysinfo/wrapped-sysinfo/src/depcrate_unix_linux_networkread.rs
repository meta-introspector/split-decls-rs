// Generated macro for read (function)
macro_rules! Depcrate_unix_linux_networkread {
() => {
// Module: crate::unix::linux::network
// Provides: {"read"}
// Dependencies: {}
# [allow (clippy :: ptr_arg)] fn read < P : AsRef < Path > > (parent : P , path : & str , data : & mut Vec < u8 >) -> u64 { if let Ok (mut f) = File :: open (parent . as_ref () . join (path)) && let Ok (size) = f . read (data) { let mut i = 0 ; let mut ret = 0 ; while i < size && i < data . len () && data [i] >= b'0' && data [i] <= b'9' { ret *= 10 ; ret += (data [i] - b'0') as u64 ; i += 1 ; } return ret ; } 0 }
};
}
