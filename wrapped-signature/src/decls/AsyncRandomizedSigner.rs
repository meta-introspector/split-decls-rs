macro_rules! deps {
    () => {
        Error!();
        Result!();
    };
}

macro_rules! AsyncRandomizedSigner {
    () => {
        deps!();
        # [doc = " Sign the given message using the provided external randomness source."] # [cfg (feature = "rand_core")] pub trait AsyncRandomizedSigner < S > { # [doc = " Sign the given message and return a digital signature"] async fn sign_with_rng_async < R : CryptoRng + ? Sized > (& self , rng : & mut R , msg : & [u8]) -> S { self . try_sign_with_rng_async (rng , msg) . await . expect ("signature operation failed") } # [doc = " Attempt to sign the given message, returning a digital signature on"] # [doc = " success, or an error if something went wrong."] # [doc = ""] # [doc = " The main intended use case for signing errors is when communicating"] # [doc = " with external signers, e.g. cloud KMS, HSMs, or other hardware tokens."] async fn try_sign_with_rng_async < R : TryCryptoRng + ? Sized > (& self , rng : & mut R , msg : & [u8] ,) -> Result < S , Error > ; }
    };
}

AsyncRandomizedSigner!()