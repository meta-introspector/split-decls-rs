macro_rules! deps {
    () => {
        RandomizedSigner!();
        Result!();
        Error!();
        AsyncRandomizedSigner!();
    };
}

macro_rules! impl_35 {
    () => {
        deps!();
        # [cfg (feature = "rand_core")] impl < S , T > AsyncRandomizedSigner < S > for T where T : RandomizedSigner < S > , { async fn try_sign_with_rng_async < R : TryCryptoRng + ? Sized > (& self , rng : & mut R , msg : & [u8] ,) -> Result < S , Error > { self . try_sign_with_rng (rng , msg) } }
    };
}

impl_35!()