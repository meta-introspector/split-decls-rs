macro_rules! deps {
    () => {
        FingerprintHasher!();
        Unhasher!();
        Fingerprint!();
    };
}

macro_rules! impl_36 {
    () => {
        deps!();
        impl FingerprintHasher for crate :: unhash :: Unhasher { # [inline] fn write_fingerprint (& mut self , fingerprint : & Fingerprint) { self . write_u64 (fingerprint . 0 . wrapping_add (fingerprint . 1)) ; } }
    };
}

impl_36!();