// Generated macro for impl_60 (impl)
macro_rules! Depcrate_kmacimpl_60 {
() => {
// Module: crate::kmac
// Provides: {"impl_60"}
// Dependencies: {}
impl Kmac { # [doc = " Creates  new [`Kmac`] hasher with a security level of 128 bits."] # [doc = ""] # [doc = " [`Kmac`]: struct.Kmac.html"] pub fn v128 (key : & [u8] , custom_string : & [u8]) -> Kmac { Kmac :: new (key , custom_string , 128) } # [doc = " Creates  new [`Kmac`] hasher with a security level of 256 bits."] # [doc = ""] # [doc = " [`Kmac`]: struct.Kmac.html"] pub fn v256 (key : & [u8] , custom_string : & [u8]) -> Kmac { Kmac :: new (key , custom_string , 256) } fn new (key : & [u8] , custom_string : & [u8] , bits : usize) -> Kmac { let rate = bits_to_rate (bits) ; let mut state = CShake :: new (b"KMAC" , custom_string , bits) ; state . update (left_encode (rate) . value ()) ; state . update (left_encode (key . len () * 8) . value ()) ; state . update (key) ; state . fill_block () ; Kmac { state } } }
};
}
