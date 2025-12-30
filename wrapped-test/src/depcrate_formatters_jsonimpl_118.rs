// Generated macro for impl_118 (impl)
macro_rules! Depcrate_formatters_jsonimpl_118 {
() => {
// Module: crate::formatters::json
// Provides: {"impl_118"}
// Dependencies: {}
impl < T : Write > JsonFormatter < T > { pub (crate) fn new (out : OutputLocation < T >) -> Self { Self { out } } fn writeln_message (& mut self , s : & str) -> io :: Result < () > { assert_eq ! (s . chars () . last () , Some ('\n')) ; self . out . write_all (s . as_ref ()) } fn write_event (& mut self , ty : & str , name : & str , event : & str , exec_time : Option < & time :: TestExecTime > , stdout : Option < Cow < '_ , str > > , extra : Option < & str > ,) -> io :: Result < () > { let name = EscapedString (name) ; let exec_time_json = if let Some (exec_time) = exec_time { format ! (r#", "exec_time": {}"# , exec_time . 0 . as_secs_f64 ()) } else { String :: from ("") } ; let stdout_json = if let Some (stdout) = stdout { format ! (r#", "stdout": "{}""# , EscapedString (stdout)) } else { String :: from ("") } ; let extra_json = if let Some (extra) = extra { format ! (r#", {extra}"#) } else { String :: from ("") } ; let newline = "\n" ; self . writeln_message (& format ! (r#"{{ "type": "{ty}", "name": "{name}", "event": "{event}"{exec_time_json}{stdout_json}{extra_json} }}{newline}"#)) } }
};
}
