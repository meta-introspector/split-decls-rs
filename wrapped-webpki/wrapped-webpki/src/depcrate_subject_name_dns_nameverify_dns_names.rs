// Generated macro for verify_dns_names (function)
macro_rules! Depcrate_subject_name_dns_nameverify_dns_names {
() => {
// Module: crate::subject_name::dns_name
// Provides: {"verify_dns_names"}
// Dependencies: {}
pub (crate) fn verify_dns_names (reference : & DnsName < '_ > , cert : & Cert < '_ >) -> Result < () , Error > { let dns_name = untrusted :: Input :: from (reference . as_ref () . as_bytes ()) ; let result = NameIterator :: new (cert . subject_alt_name) . find_map (| result | { let name = match result { Ok (name) => name , Err (err) => return Some (Err (err)) , } ; let presented_id = match name { GeneralName :: DnsName (presented) => presented , _ => return None , } ; match presented_id_matches_reference_id (presented_id , IdRole :: Reference , dns_name) { Ok (true) => Some (Ok (())) , Ok (false) | Err (Error :: MalformedDnsIdentifier) => None , Err (e) => Some (Err (e)) , } }) ; match result { Some (result) => return result , # [cfg (feature = "alloc")] None => { } # [cfg (not (feature = "alloc"))] None => Err (Error :: CertNotValidForName (InvalidNameContext { })) , } # [cfg (feature = "alloc")] { Err (Error :: CertNotValidForName (InvalidNameContext { expected : ServerName :: DnsName (reference . to_owned ()) , presented : NameIterator :: new (cert . subject_alt_name) . filter_map (| result | Some (format ! ("{:?}" , result . ok () ?))) . collect () , })) } }
};
}
