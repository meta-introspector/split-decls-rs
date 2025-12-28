macro_rules! deps {
    () => {
        Error!();
        Result!();
    };
}

macro_rules! AsyncRandomizedPrehashSigner {
    () => {
        deps!();
        # [doc = " Asynchronously sign the provided message prehash using the provided external randomness source, returning a digital signature."] # [cfg (feature = "rand_core")] # [allow (async_fn_in_trait)] pub trait AsyncRandomizedPrehashSigner < S > { # [doc = " Attempt to sign the given message digest, returning a digital signature"] # [doc = " on success, or an error if something went wrong."] # [doc = ""] # [doc = " The `prehash` parameter should be the output of a secure cryptographic"] # [doc = " hash function."] # [doc = ""] # [doc = " This API takes a `prehash` byte slice as there can potentially be many"] # [doc = " compatible lengths for the message digest for a given concrete signature"] # [doc = " algorithm."] # [doc = ""] # [doc = " Allowed lengths are algorithm-dependent and up to a particular"] # [doc = " implementation to decide."] async fn sign_prehash_with_rng_async < R : TryCryptoRng + ? Sized > (& self , rng : & mut R , prehash : & [u8] ,) -> Result < S , Error > ; }
    };
}

AsyncRandomizedPrehashSigner!()