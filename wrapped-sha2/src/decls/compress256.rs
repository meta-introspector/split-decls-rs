macro_rules! compress256 {
    () => {
        # [doc = " Raw SHA-256 compression function."] # [doc = ""] # [doc = " This is a low-level \"hazmat\" API which provides direct access to the core"] # [doc = " functionality of SHA-256."] pub fn compress256 (state : & mut [u32 ; 8] , blocks : & [[u8 ; 64]]) { compress (state , blocks) }
    };
}

compress256!();