// Generated macro for set_percentage_stat (function)
macro_rules! Depcrate_serverset_percentage_stat {
() => {
// Module: crate::server
// Provides: {"set_percentage_stat"}
// Dependencies: {}
fn set_percentage_stat (vec : & mut Vec < (String , String , usize) > , count_hits : u64 , total : u64 , name : & str ,) { if total == 0 { vec . push ((name . to_string () , "-" . to_string () , 0)) ; } else { let ratio = count_hits as f64 / total as f64 ; vec . push ((name . to_string () , format ! ("{:.2} %" , ratio * 100.0) , 2)) ; } }
};
}
