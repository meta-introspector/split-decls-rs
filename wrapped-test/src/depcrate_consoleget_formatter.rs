// Generated macro for get_formatter (function)
macro_rules! Depcrate_consoleget_formatter {
() => {
// Module: crate::console
// Provides: {"get_formatter"}
// Dependencies: {}
pub (crate) fn get_formatter (opts : & TestOpts , max_name_len : usize) -> Box < dyn OutputFormatter > { let output = match term :: stdout () { None => OutputLocation :: Raw (io :: stdout ()) , Some (t) => OutputLocation :: Pretty (t) , } ; let is_multithreaded = opts . test_threads . unwrap_or_else (get_concurrency) > 1 ; match opts . format { OutputFormat :: Pretty => Box :: new (PrettyFormatter :: new (output , opts . use_color () , max_name_len , is_multithreaded , opts . time_options ,)) , OutputFormat :: Terse => { Box :: new (TerseFormatter :: new (output , opts . use_color () , max_name_len , is_multithreaded)) } OutputFormat :: Json => Box :: new (JsonFormatter :: new (output)) , OutputFormat :: Junit => Box :: new (JunitFormatter :: new (output)) , } }
};
}
