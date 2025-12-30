// Generated macro for impl_92 (impl)
macro_rules! Depcrateimpl_92 {
() => {
// Module: crate
// Provides: {"impl_92"}
// Dependencies: {}
impl Update for Sha1 { # [inline] fn update (& mut self , input : & [u8]) { let Self { h , detection , buffer , .. } = self ; buffer . digest_blocks (input , | blocks | { self . block_len += blocks . len () as u64 ; if let Some (ctx) = detection { let blocks : & [[u8 ; BLOCK_SIZE]] = unsafe { & * (blocks as * const _ as * const [[u8 ; BLOCK_SIZE]]) } ; compress :: compress (h , ctx , blocks) ; } else { let blocks = Array :: cast_slice_to_core (blocks) ; sha1 :: block_api :: compress (h , blocks) ; } }) ; } }
};
}
