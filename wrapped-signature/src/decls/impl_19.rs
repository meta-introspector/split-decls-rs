macro_rules! deps {
    () => {
        Keypair!();
        KeypairRef!();
    };
}

macro_rules! impl_19 {
    () => {
        deps!();
        impl < K : KeypairRef > Keypair for K { type VerifyingKey = < Self as KeypairRef > :: VerifyingKey ; fn verifying_key (& self) -> Self :: VerifyingKey { self . as_ref () . clone () } }
    };
}

impl_19!();