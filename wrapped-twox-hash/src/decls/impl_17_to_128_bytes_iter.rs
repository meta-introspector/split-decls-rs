macro_rules! impl_17_to_128_bytes_iter {
    () => {
        # [inline] pub fn impl_17_to_128_bytes_iter (secret : & Secret , input : & [u8] , mut f : impl FnMut (& [u8 ; 16] , & [u8 ; 16] , & [[u8 ; 16] ; 2]) ,) { let secret = secret . words_for_17_to_128 () ; let (secret , _) = secret . bp_as_chunks :: < 2 > () ; let (fwd , _) = input . bp_as_chunks () ; let (_ , bwd) = input . bp_as_rchunks () ; let q = bwd . len () ; if input . len () > 32 { if input . len () > 64 { if input . len () > 96 { f (& fwd [3] , & bwd [q - 4] , & secret [3]) ; } f (& fwd [2] , & bwd [q - 3] , & secret [2]) ; } f (& fwd [1] , & bwd [q - 2] , & secret [1]) ; } f (& fwd [0] , & bwd [q - 1] , & secret [0]) ; }
    };
}

impl_17_to_128_bytes_iter!()