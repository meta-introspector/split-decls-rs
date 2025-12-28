use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mkdeclimpl! {
impl Drop for CpuSpan { fn drop (& mut self) { # [cfg (feature = "cpu_profiler")] { google_cpu_profiler :: stop () ; let profile_data = std :: env :: current_dir () . unwrap () . join ("out.profile") ; eprintln ! ("Profile data saved to:\n\n    {}\n" , profile_data . display ()) ; let mut cmd = std :: process :: Command :: new ("pprof") ; cmd . arg ("-svg") . arg (std :: env :: current_exe () . unwrap ()) . arg (& profile_data) ; let out = cmd . output () ; match out { Ok (out) if out . status . success () => { let svg = profile_data . with_extension ("svg") ; std :: fs :: write (& svg , out . stdout) . unwrap () ; eprintln ! ("Profile rendered to:\n\n    {}\n" , svg . display ()) ; } _ => { eprintln ! ("Failed to run:\n\n   {cmd:?}\n") ; } } } } }
}