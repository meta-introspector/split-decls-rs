// Generated macro for impl_374 (impl)
macro_rules! Depcrate_x509impl_374 {
() => {
// Module: crate::x509
// Provides: {"impl_374"}
// Dependencies: {}
impl < 'a > FromDer < 'a > for DistributionPointName < 'a > { fn from_der (reader : & mut untrusted :: Reader < 'a >) -> Result < Self , Error > { const FULL_NAME_TAG : u8 = CONTEXT_SPECIFIC | CONSTRUCTED ; const NAME_RELATIVE_TO_CRL_ISSUER_TAG : u8 = CONTEXT_SPECIFIC | CONSTRUCTED | 1 ; let (tag , value) = der :: read_tag_and_get_value (reader) ? ; match tag { FULL_NAME_TAG => Ok (DistributionPointName :: FullName (DerIterator :: new (value))) , NAME_RELATIVE_TO_CRL_ISSUER_TAG => Ok (DistributionPointName :: NameRelativeToCrlIssuer) , _ => Err (Error :: BadDer) , } } const TYPE_ID : DerTypeId = DerTypeId :: DistributionPointName ; }
};
}
