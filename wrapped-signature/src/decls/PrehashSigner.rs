macro_rules! deps {
    () => {
        Error!();
        Result!();
    };
}

macro_rules! PrehashSigner {
    () => {
        deps!();
        # [doc = " Sign the provided message prehash, returning a digital signature."] pub trait PrehashSigner < S > { # [doc = " Attempt to sign the given message digest, returning a digital signature"] # [doc = " on success, or an error if something went wrong."] # [doc = ""] # [doc = " The `prehash` parameter should be the output of a secure cryptographic"] # [doc = " hash function."] # [doc = ""] # [doc = " This API takes a `prehash` byte slice as there can potentially be many"] # [doc = " compatible lengths for the message digest for a given concrete signature"] # [doc = " algorithm."] # [doc = ""] # [doc = " Allowed lengths are algorithm-dependent and up to a particular"] # [doc = " implementation to decide."] fn sign_prehash (& self , prehash : & [u8]) -> Result < S , Error > ; }
    };
}

PrehashSigner!();