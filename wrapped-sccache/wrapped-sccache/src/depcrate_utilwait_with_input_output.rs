// Generated macro for wait_with_input_output (function)
macro_rules! Depcrate_utilwait_with_input_output {
() => {
// Module: crate::util
// Provides: {"wait_with_input_output"}
// Dependencies: {}
# [doc = " If `input`, write it to `child`'s stdin while also reading `child`'s stdout and stderr, then wait on `child` and return its status and output."] # [doc = ""] # [doc = " This was lifted from `std::process::Child::wait_with_output` and modified"] # [doc = " to also write to stdin."] async fn wait_with_input_output < T > (mut child : T , input : Option < Vec < u8 > >) -> Result < process :: Output > where T : CommandChild + 'static , { use tokio :: io :: { AsyncReadExt , AsyncWriteExt } ; let stdin = input . and_then (| i | { child . take_stdin () . map (| mut stdin | async move { stdin . write_all (& i) . await . context ("failed to write stdin") }) }) ; let stdout = child . take_stdout () ; let stdout = async move { match stdout { Some (mut stdout) => { let mut buf = Vec :: new () ; stdout . read_to_end (& mut buf) . await . context ("failed to read stdout") ? ; Result :: Ok (Some (buf)) } None => Ok (None) , } } ; let stderr = child . take_stderr () ; let stderr = async move { match stderr { Some (mut stderr) => { let mut buf = Vec :: new () ; stderr . read_to_end (& mut buf) . await . context ("failed to read stderr") ? ; Result :: Ok (Some (buf)) } None => Ok (None) , } } ; let status = async move { if let Some (stdin) = stdin { let _ = stdin . await ; } child . wait () . await . context ("failed to wait for child") } ; let (status , stdout , stderr) = futures :: future :: try_join3 (status , stdout , stderr) . await ? ; Ok (process :: Output { status , stdout : stdout . unwrap_or_default () , stderr : stderr . unwrap_or_default () , }) }
};
}
