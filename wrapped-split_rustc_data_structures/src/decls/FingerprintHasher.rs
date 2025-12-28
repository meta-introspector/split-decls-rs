macro_rules! deps {
    () => {
        Fingerprint!();
    };
}

macro_rules! FingerprintHasher {
    () => {
        deps!();
        trait FingerprintHasher { fn write_fingerprint (& mut self , fingerprint : & Fingerprint) ; }
    };
}

FingerprintHasher!()