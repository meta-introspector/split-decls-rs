// Generated macro for impl_16 (impl)
macro_rules! Depcrate_schemaimpl_16 {
() => {
// Module: crate::schema
// Provides: {"impl_16"}
// Dependencies: {}
impl From < OneShot > for TryCmd { fn from (other : OneShot) -> Self { let OneShot { bin , args , env , stdin , stdout , stderr , stderr_to_stdout , status , binary , timeout , fs , } = other ; Self { steps : vec ! [Step { id : None , bin , args : args . into_vec () , env , stdin : stdin . map (crate :: Data :: text) , stderr_to_stdout , expected_status_source : None , expected_status : status , expected_stdout_source : None , expected_stdout : stdout . map (crate :: Data :: text) , expected_stderr_source : None , expected_stderr : stderr . map (crate :: Data :: text) , binary , timeout , }] , fs , } } }
};
}
