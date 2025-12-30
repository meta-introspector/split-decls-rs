// Generated macro for check_basic_constraints (function)
macro_rules! Depcrate_verify_certcheck_basic_constraints {
() => {
// Module: crate::verify_cert
// Provides: {"check_basic_constraints"}
// Dependencies: {}
fn check_basic_constraints (input : Option < & mut untrusted :: Reader < '_ > > , role : Role , sub_ca_count : usize ,) -> Result < () , Error > { let (is_ca , path_len_constraint) = match input { Some (input) => { let is_ca = bool :: from_der (input) ? ; let path_len_constraint = if ! input . at_end () { Some (usize :: from (u8 :: from_der (input) ?)) } else { None } ; (is_ca , path_len_constraint) } None => (false , None) , } ; match (role , is_ca , path_len_constraint) { (Role :: EndEntity , true , _) => Err (Error :: CaUsedAsEndEntity) , (Role :: Issuer , false , _) => Err (Error :: EndEntityUsedAsCa) , (Role :: Issuer , true , Some (len)) if sub_ca_count > len => { Err (Error :: PathLenConstraintViolated) } _ => Ok (()) , } }
};
}
