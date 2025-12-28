macro_rules! deps {
    () => {
        EcPrivateKey!();
        Error!();
        Result!();
    };
}

macro_rules! impl_57 {
    () => {
        deps!();
        # [cfg (feature = "alloc")] impl TryFrom < & EcPrivateKey < '_ > > for SecretDocument { type Error = Error ; fn try_from (private_key : & EcPrivateKey < '_ >) -> Result < Self > { Ok (Self :: encode_msg (private_key) ?) } }
    };
}

impl_57!()