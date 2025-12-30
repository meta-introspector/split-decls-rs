// Generated macro for impl_26 (impl)
macro_rules! Depcrate_k12impl_26 {
() => {
// Module: crate::k12
// Provides: {"impl_26"}
// Dependencies: {}
impl < T : AsRef < [u8] > > Hasher for KangarooTwelve < T > { fn update (& mut self , input : & [u8]) { let mut to_absorb = input ; if self . chunks == 0 { let todo = core :: cmp :: min (Self :: MAX_CHUNK_SIZE - self . written , to_absorb . len ()) ; self . state . update (& to_absorb [.. todo]) ; self . written += todo ; to_absorb = & to_absorb [todo ..] ; if to_absorb . len () > 0 && self . written == Self :: MAX_CHUNK_SIZE { self . state . update (& [0x03 , 0 , 0 , 0 , 0 , 0 , 0 , 0]) ; self . written = 0 ; self . chunks += 1 ; } } while to_absorb . len () > 0 { if self . written == Self :: MAX_CHUNK_SIZE { let mut chunk_hash = [0u8 ; 32] ; let current_chunk = self . current_chunk . clone () ; self . current_chunk . reset () ; current_chunk . finalize (& mut chunk_hash) ; self . state . update (& chunk_hash) ; self . written = 0 ; self . chunks += 1 ; } let todo = core :: cmp :: min (Self :: MAX_CHUNK_SIZE - self . written , to_absorb . len ()) ; self . current_chunk . update (& to_absorb [.. todo]) ; self . written += todo ; to_absorb = & to_absorb [todo ..] ; } } fn finalize (self , output : & mut [u8]) { let mut xof = self . into_xof () ; xof . squeeze (output) ; } }
};
}
