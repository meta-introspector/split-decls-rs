// Generated macro for verify_ip_address_names (function)
macro_rules! Depcrate_subject_name_ip_addressverify_ip_address_names {
() => {
// Module: crate::subject_name::ip_address
// Provides: {"verify_ip_address_names"}
// Dependencies: {}
pub (crate) fn verify_ip_address_names (reference : & IpAddr , cert : & Cert < '_ >) -> Result < () , Error > { let ip_address = match reference { IpAddr :: V4 (ip) => untrusted :: Input :: from (ip . as_ref ()) , IpAddr :: V6 (ip) => untrusted :: Input :: from (ip . as_ref ()) , } ; let result = NameIterator :: new (cert . subject_alt_name) . find_map (| result | { let name = match result { Ok (name) => name , Err (err) => return Some (Err (err)) , } ; let presented_id = match name { GeneralName :: IpAddress (presented) => presented , _ => return None , } ; match presented_id_matches_reference_id (presented_id , ip_address) { true => Some (Ok (())) , false => None , } }) ; match result { Some (result) => return result , # [cfg (feature = "alloc")] None => { } # [cfg (not (feature = "alloc"))] None => Err (Error :: CertNotValidForName (InvalidNameContext { })) , } # [cfg (feature = "alloc")] { Err (Error :: CertNotValidForName (InvalidNameContext { expected : ServerName :: from (* reference) , presented : NameIterator :: new (cert . subject_alt_name) . filter_map (| result | Some (format ! ("{:?}" , result . ok () ?))) . collect () , })) } }
};
}
