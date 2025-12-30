// Generated macro for impl_1152 (impl)
macro_rules! Depcrate_distimpl_1152 {
() => {
// Module: crate::dist
// Provides: {"impl_1152"}
// Dependencies: {}
impl ProcessOutput { # [cfg (unix)] pub fn try_from (o : process :: Output) -> Result < Self > { let process :: Output { status , stdout , stderr , } = o ; let code = match (status . code () , status . signal ()) { (Some (c) , _) => c , (None , Some (s)) => bail ! ("Process status {} terminated with signal {}" , status , s) , (None , None) => bail ! ("Process status {} has no exit code or signal" , status) , } ; Ok (ProcessOutput { code , stdout , stderr , }) } # [cfg (test)] pub fn fake_output (code : i32 , stdout : Vec < u8 > , stderr : Vec < u8 >) -> Self { Self { code , stdout , stderr , } } }
};
}
