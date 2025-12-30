// Generated macro for presented_id_matches_reference_id (function)
macro_rules! Depcrate_subject_name_ip_addresspresented_id_matches_reference_id {
() => {
// Module: crate::subject_name::ip_address
// Provides: {"presented_id_matches_reference_id"}
// Dependencies: {}
fn presented_id_matches_reference_id (presented_id : untrusted :: Input < '_ > , reference_id : untrusted :: Input < '_ > ,) -> bool { match (presented_id . len () , reference_id . len ()) { (4 , 4) => () , (16 , 16) => () , _ => { return false ; } } ; let mut presented_ip_address = untrusted :: Reader :: new (presented_id) ; let mut reference_ip_address = untrusted :: Reader :: new (reference_id) ; while ! presented_ip_address . at_end () { let presented_ip_address_byte = presented_ip_address . read_byte () . unwrap () ; let reference_ip_address_byte = reference_ip_address . read_byte () . unwrap () ; if presented_ip_address_byte != reference_ip_address_byte { return false ; } } true }
};
}
