// Generated macro for impl_28 (impl)
macro_rules! Depcrate_k12impl_28 {
() => {
// Module: crate::k12
// Provides: {"impl_28"}
// Dependencies: {}
impl < T : AsRef < [u8] > > IntoXof for KangarooTwelve < T > { type Xof = KangarooTwelveXof ; fn into_xof (mut self) -> KangarooTwelveXof { let custom_string = self . custom_string . take () . expect ("KangarooTwelve cannot be initialized without custom_string; qed") ; let encoded_len = encode_len (custom_string . as_ref () . len ()) ; self . update (custom_string . as_ref ()) ; self . update (encoded_len . value ()) ; if self . chunks == 0 { self . state . delim = 0x07 ; } else { let encoded_chunks = encode_len (self . chunks) ; let mut tmp_chunk = [0u8 ; 32] ; self . current_chunk . finalize (& mut tmp_chunk) ; self . state . update (& tmp_chunk) ; self . state . update (encoded_chunks . value ()) ; self . state . update (& [0xff , 0xff]) ; self . state . delim = 0x06 ; } KangarooTwelveXof { state : self . state } } }
};
}
