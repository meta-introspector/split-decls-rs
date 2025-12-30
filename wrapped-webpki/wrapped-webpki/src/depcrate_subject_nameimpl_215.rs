// Generated macro for impl_215 (impl)
macro_rules! Depcrate_subject_nameimpl_215 {
() => {
// Module: crate::subject_name
// Provides: {"impl_215"}
// Dependencies: {}
impl < 'a > FromDer < 'a > for GeneralName < 'a > { fn from_der (reader : & mut untrusted :: Reader < 'a >) -> Result < Self , Error > { use GeneralName :: * ; use der :: { CONSTRUCTED , CONTEXT_SPECIFIC } ; # [allow (clippy :: identity_op)] const OTHER_NAME_TAG : u8 = CONTEXT_SPECIFIC | CONSTRUCTED | 0 ; const RFC822_NAME_TAG : u8 = CONTEXT_SPECIFIC | 1 ; const DNS_NAME_TAG : u8 = CONTEXT_SPECIFIC | 2 ; const X400_ADDRESS_TAG : u8 = CONTEXT_SPECIFIC | CONSTRUCTED | 3 ; const DIRECTORY_NAME_TAG : u8 = CONTEXT_SPECIFIC | CONSTRUCTED | 4 ; const EDI_PARTY_NAME_TAG : u8 = CONTEXT_SPECIFIC | CONSTRUCTED | 5 ; const UNIFORM_RESOURCE_IDENTIFIER_TAG : u8 = CONTEXT_SPECIFIC | 6 ; const IP_ADDRESS_TAG : u8 = CONTEXT_SPECIFIC | 7 ; const REGISTERED_ID_TAG : u8 = CONTEXT_SPECIFIC | 8 ; let (tag , value) = der :: read_tag_and_get_value (reader) ? ; Ok (match tag { DNS_NAME_TAG => DnsName (value) , DIRECTORY_NAME_TAG => DirectoryName , IP_ADDRESS_TAG => IpAddress (value) , UNIFORM_RESOURCE_IDENTIFIER_TAG => UniformResourceIdentifier (value) , OTHER_NAME_TAG | RFC822_NAME_TAG | X400_ADDRESS_TAG | EDI_PARTY_NAME_TAG | REGISTERED_ID_TAG => Unsupported (tag & ! (CONTEXT_SPECIFIC | CONSTRUCTED)) , _ => return Err (Error :: BadDer) , }) } const TYPE_ID : DerTypeId = DerTypeId :: GeneralName ; }
};
}
