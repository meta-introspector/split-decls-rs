macro_rules! Keypair {
    () => {
        # [doc = " Signing keypair with an associated verifying key."] # [doc = ""] # [doc = " This represents a type which holds both a signing key and a verifying key."] pub trait Keypair { # [doc = " Verifying key type for this keypair."] type VerifyingKey : Clone ; # [doc = " Get the verifying key which can verify signatures produced by the"] # [doc = " signing key portion of this keypair."] fn verifying_key (& self) -> Self :: VerifyingKey ; }
    };
}

Keypair!()