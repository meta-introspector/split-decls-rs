// Generated macro for output (function)
macro_rules! Depcrate_sys_processoutput {
() => {
// Module: crate::sys::process
// Provides: {"output"}
// Dependencies: {}
# [cfg (any (all (target_family = "unix" , not (any (target_os = "espidf" , target_os = "horizon" , target_os = "vita" , target_os = "nuttx"))) , target_os = "windows" ,))] pub fn output (cmd : & mut Command) -> crate :: io :: Result < (ExitStatus , Vec < u8 > , Vec < u8 >) > { use crate :: sys :: pipe :: read2 ; let (mut process , mut pipes) = cmd . spawn (Stdio :: MakePipe , false) ? ; drop (pipes . stdin . take ()) ; let (mut stdout , mut stderr) = (Vec :: new () , Vec :: new ()) ; match (pipes . stdout . take () , pipes . stderr . take ()) { (None , None) => { } (Some (out) , None) => { let res = out . read_to_end (& mut stdout) ; res . unwrap () ; } (None , Some (err)) => { let res = err . read_to_end (& mut stderr) ; res . unwrap () ; } (Some (out) , Some (err)) => { let res = read2 (out , & mut stdout , err , & mut stderr) ; res . unwrap () ; } } let status = process . wait () ? ; Ok ((status , stdout , stderr)) }
};
}
