macro_rules! deps {
    () => {
        EcParameters!();
        EcPrivateKey!();
        Result!();
    };
}

macro_rules! impl_50 {
    () => {
        deps!();
        impl < 'a > EcPrivateKey < 'a > { fn context_specific_parameters (& self) -> Option < ContextSpecificRef < '_ , EcParameters > > { self . parameters . as_ref () . map (| params | ContextSpecificRef { tag_number : EC_PARAMETERS_TAG , tag_mode : TagMode :: Explicit , value : params , }) } fn context_specific_public_key (& self ,) -> der :: Result < Option < ContextSpecific < BitStringRef < 'a > > > > { self . public_key . map (| pk | { BitStringRef :: from_bytes (pk) . map (| value | ContextSpecific { tag_number : PUBLIC_KEY_TAG , tag_mode : TagMode :: Explicit , value , }) }) . transpose () } }
    };
}

impl_50!();