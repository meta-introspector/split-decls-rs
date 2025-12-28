macro_rules! deps {
    () => {
        Error!();
        Result!();
        RandomizedSigner!();
        RandomizedSignerMut!();
    };
}

macro_rules! impl_30 {
    () => {
        deps!();
        # [doc = " Blanket impl of [`RandomizedSignerMut`] for all [`RandomizedSigner`] types."] # [cfg (feature = "rand_core")] impl < S , T : RandomizedSigner < S > > RandomizedSignerMut < S > for T { fn try_sign_with_rng < R : TryCryptoRng + ? Sized > (& mut self , rng : & mut R , msg : & [u8] ,) -> Result < S , Error > { T :: try_sign_with_rng (self , rng , msg) } }
    };
}

impl_30!();