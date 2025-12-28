macro_rules! deps {
    () => {
        EcPrivateKey!();
        Error!();
        Result!();
    };
}

macro_rules! impl_56 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl TryFrom < EcPrivateKey < '_ > > for SecretDocument { type Error = Error ; fn try_from (private_key : EcPrivateKey < '_ >) -> Result < Self > { SecretDocument :: try_from (& private_key) } }
    };
}

impl_56!()