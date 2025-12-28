macro_rules! deps {
    () => {
        Error!();
        AsyncSigner!();
        Result!();
        Signer!();
    };
}

macro_rules! impl_32 {
    () => {
        deps!();
        impl < S , T > AsyncSigner < S > for T where T : Signer < S > , { async fn sign_async (& self , msg : & [u8]) -> Result < S , Error > { self . try_sign (msg) } }
    };
}

impl_32!();