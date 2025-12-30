// Generated macro for ignore_notfound (function)
macro_rules! Depcrate_sys_commonignore_notfound {
() => {
// Module: crate::sys_common
// Provides: {"ignore_notfound"}
// Dependencies: {}
pub fn ignore_notfound < T > (result : crate :: io :: Result < T >) -> crate :: io :: Result < () > { match result { Err (err) if err . kind () == crate :: io :: ErrorKind :: NotFound => Ok (()) , Ok (_) => Ok (()) , Err (err) => Err (err) , } }
};
}
