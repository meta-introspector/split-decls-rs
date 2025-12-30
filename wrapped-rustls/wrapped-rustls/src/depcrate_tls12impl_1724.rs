// Generated macro for impl_1724 (impl)
macro_rules! Depcrate_tls12impl_1724 {
() => {
// Module: crate::tls12
// Provides: {"impl_1724"}
// Dependencies: {}
impl Exporter for Tls12Exporter { fn derive (& self , label : & [u8] , context : Option < & [u8] > , output : & mut [u8]) -> Result < () , Error > { let mut randoms = Vec :: with_capacity (32 + 32 + context . as_ref () . map (| c | 2 + c . len ()) . unwrap_or_default () ,) ; randoms . extend_from_slice (& self . randoms . client) ; randoms . extend_from_slice (& self . randoms . server) ; if let Some (context) = context { match u16 :: try_from (context . len ()) { Ok (len) => len . encode (& mut randoms) , Err (_) => return Err (ApiMisuse :: ExporterContextTooLong . into ()) , } randoms . extend_from_slice (context) ; } self . master_secret_prf . prf (output , label , & randoms) ; Ok (()) } }
};
}
