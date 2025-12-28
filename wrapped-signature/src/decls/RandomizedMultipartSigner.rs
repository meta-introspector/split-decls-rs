macro_rules! deps {
    () => {
        Error!();
        RandomizedSigner!();
        Result!();
    };
}

macro_rules! RandomizedMultipartSigner {
    () => {
        deps!();
        # [doc = " Equivalent of [`RandomizedSigner`] but the message is provided in non-contiguous byte slices."] # [cfg (feature = "rand_core")] pub trait RandomizedMultipartSigner < S > { # [doc = " Equivalent of [`RandomizedSigner::sign_with_rng()`] but"] # [doc = " the message is provided in non-contiguous byte slices."] fn multipart_sign_with_rng < R : CryptoRng + ? Sized > (& self , rng : & mut R , msg : & [& [u8]]) -> S { self . try_multipart_sign_with_rng (rng , msg) . expect ("signature operation failed") } # [doc = " Equivalent of [`RandomizedSigner::try_sign_with_rng()`] but"] # [doc = " the message is provided in non-contiguous byte slices."] fn try_multipart_sign_with_rng < R : TryCryptoRng + ? Sized > (& self , rng : & mut R , msg : & [& [u8]] ,) -> Result < S , Error > ; }
    };
}

RandomizedMultipartSigner!();