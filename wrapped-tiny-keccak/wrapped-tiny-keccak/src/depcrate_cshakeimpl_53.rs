// Generated macro for impl_53 (impl)
macro_rules! Depcrate_cshakeimpl_53 {
() => {
// Module: crate::cshake
// Provides: {"impl_53"}
// Dependencies: {}
impl CShake { const DELIM : u8 = 0x04 ; # [doc = " Creates  new [`CShake`] hasher with a security level of 128 bits."] # [doc = ""] # [doc = " [`CShake`]: struct.CShake.html"] pub fn v128 (name : & [u8] , custom_string : & [u8]) -> CShake { CShake :: new (name , custom_string , 128) } # [doc = " Creates  new [`CShake`] hasher with a security level of 256 bits."] # [doc = ""] # [doc = " [`CShake`]: struct.CShake.html"] pub fn v256 (name : & [u8] , custom_string : & [u8]) -> CShake { CShake :: new (name , custom_string , 256) } pub (crate) fn new (name : & [u8] , custom_string : & [u8] , bits : usize) -> CShake { let rate = bits_to_rate (bits) ; if name . is_empty () && custom_string . is_empty () { let state = KeccakState :: new (rate , 0x1f) ; return CShake { state } ; } let mut state = KeccakState :: new (rate , Self :: DELIM) ; state . update (left_encode (rate) . value ()) ; state . update (left_encode (name . len () * 8) . value ()) ; state . update (name) ; state . update (left_encode (custom_string . len () * 8) . value ()) ; state . update (custom_string) ; state . fill_block () ; CShake { state } } pub (crate) fn fill_block (& mut self) { self . state . fill_block () ; } }
};
}
