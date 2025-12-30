// Generated macro for impl_25 (impl)
macro_rules! Depcrate_k12impl_25 {
() => {
// Module: crate::k12
// Provides: {"impl_25"}
// Dependencies: {}
impl < T > KangarooTwelve < T > { const MAX_CHUNK_SIZE : usize = 8192 ; # [doc = " Creates  new [`KangarooTwelve`] hasher with a security level of 128 bits."] # [doc = ""] # [doc = " [`KangarooTwelve`]: struct.KangarooTwelve.html"] pub fn new (custom_string : T) -> Self { let rate = bits_to_rate (128) ; KangarooTwelve { state : KeccakState :: new (rate , 0) , current_chunk : KeccakState :: new (rate , 0x0b) , custom_string : Some (custom_string) , written : 0 , chunks : 0 , } } }
};
}
