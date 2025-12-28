macro_rules! deps {
    () => {
        Result!();
        Error!();
        EcPrivateKey!();
        Tag!();
    };
}

macro_rules! impl_51 {
    () => {
        deps!();
        impl < 'a > DecodeValue < 'a > for EcPrivateKey < 'a > { type Error = der :: Error ; fn decode_value < R : Reader < 'a > > (reader : & mut R , _header : Header) -> der :: Result < Self > { if u8 :: decode (reader) ? != VERSION { return Err (reader . error (Tag :: Integer . value_error ())) ; } let private_key = < & OctetStringRef > :: decode (reader) ? . as_bytes () ; let parameters = reader . context_specific (EC_PARAMETERS_TAG , TagMode :: Explicit) ? ; let public_key = reader . context_specific :: < BitStringRef < '_ > > (PUBLIC_KEY_TAG , TagMode :: Explicit) ? . map (| bs | bs . as_bytes () . ok_or_else (| | Tag :: BitString . value_error ())) . transpose () ? ; Ok (EcPrivateKey { private_key , parameters , public_key , }) } }
    };
}

impl_51!()