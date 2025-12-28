macro_rules! deps {
    () => {
        Hasher!();
        OneshotWithSecretError!();
    };
}

macro_rules! impl_96 {
    () => {
        deps!();
        impl Hasher { # [doc = " Hash all data at once. If you can use this function, you may"] # [doc = " see noticable speed gains for certain types of input."] # [must_use] # [inline] pub fn oneshot (input : & [u8]) -> u64 { impl_oneshot (DEFAULT_SECRET , DEFAULT_SEED , input) } # [doc = " Hash all data at once using the provided seed and a secret"] # [doc = " derived from the seed. If you can use this function, you may"] # [doc = " see noticable speed gains for certain types of input."] # [must_use] # [inline] pub fn oneshot_with_seed (seed : u64 , input : & [u8]) -> u64 { let mut secret = DEFAULT_SECRET_RAW ; if input . len () > CUTOFF { derive_secret (seed , & mut secret) ; } let secret = Secret :: new (& secret) . expect ("The default secret length is invalid") ; impl_oneshot (secret , seed , input) } # [doc = " Hash all data at once using the provided secret and the"] # [doc = " default seed. If you can use this function, you may see"] # [doc = " noticable speed gains for certain types of input."] # [inline] pub fn oneshot_with_secret (secret : & [u8] , input : & [u8]) -> Result < u64 , OneshotWithSecretError > { let secret = Secret :: new (secret) . map_err (OneshotWithSecretError) ? ; Ok (impl_oneshot (secret , DEFAULT_SEED , input)) } # [doc = " Hash all data at once using the provided seed and secret. If"] # [doc = " you can use this function, you may see noticable speed gains"] # [doc = " for certain types of input."] # [inline] pub fn oneshot_with_seed_and_secret (seed : u64 , secret : & [u8] , input : & [u8] ,) -> Result < u64 , OneshotWithSecretError > { let secret = if input . len () > CUTOFF { Secret :: new (secret) . map_err (OneshotWithSecretError) ? } else { DEFAULT_SECRET } ; Ok (impl_oneshot (secret , seed , input)) } }
    };
}

impl_96!();