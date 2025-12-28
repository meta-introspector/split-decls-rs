macro_rules! deps {
    () => {
        RawHasher!();
    };
}

macro_rules! impl_118 {
    () => {
        deps!();
        impl < S > RawHasher < S > { # [doc = " Construct the hasher with the provided seed, secret, and"] # [doc = " temporary buffer."] pub fn new (secret_buffer : SecretBuffer < S >) -> Self { Self (RawHasherCore :: new (secret_buffer)) } # [doc = " Returns the secret."] pub fn into_secret (self) -> S { self . 0 . into_secret () } }
    };
}

impl_118!()