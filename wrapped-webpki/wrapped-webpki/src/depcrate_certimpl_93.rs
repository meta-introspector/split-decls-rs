// Generated macro for impl_93 (impl)
macro_rules! Depcrate_certimpl_93 {
() => {
// Module: crate::cert
// Provides: {"impl_93"}
// Dependencies: {}
impl < 'a > FromDer < 'a > for CrlDistributionPoint < 'a > { fn from_der (reader : & mut untrusted :: Reader < 'a >) -> Result < Self , Error > { let mut result = CrlDistributionPoint { distribution_point : None , reasons : None , crl_issuer : None , } ; der :: nested (reader , Tag :: Sequence , Error :: TrailingData (Self :: TYPE_ID) , | der | { const DISTRIBUTION_POINT_TAG : u8 = CONTEXT_SPECIFIC | CONSTRUCTED ; const REASONS_TAG : u8 = CONTEXT_SPECIFIC | 1 ; const CRL_ISSUER_TAG : u8 = CONTEXT_SPECIFIC | CONSTRUCTED | 2 ; while ! der . at_end () { let (tag , value) = der :: read_tag_and_get_value (der) ? ; match tag { DISTRIBUTION_POINT_TAG => { set_extension_once (& mut result . distribution_point , | | Ok (value)) ? } REASONS_TAG => set_extension_once (& mut result . reasons , | | { der :: bit_string_flags (value) }) ? , CRL_ISSUER_TAG => set_extension_once (& mut result . crl_issuer , | | Ok (value)) ? , _ => return Err (Error :: BadDer) , } } match (result . distribution_point , result . crl_issuer) { (None , None) => Err (Error :: MalformedExtensions) , _ => Ok (result) , } } ,) } const TYPE_ID : DerTypeId = DerTypeId :: CrlDistributionPoint ; }
};
}
