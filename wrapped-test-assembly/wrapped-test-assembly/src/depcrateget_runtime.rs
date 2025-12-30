// Generated macro for get_runtime (function)
macro_rules! Depcrateget_runtime {
() => {
// Module: crate
// Provides: {"get_runtime"}
// Dependencies: {}
pub fn get_runtime () -> String { use regex :: Regex ; let re = Regex :: new (r"--features[= ]+(([a-z0-9_-]+,?)+)") . unwrap () ; let combined = std :: env :: args () . collect :: < Vec < _ > > () . join (" ") ; let result = re . captures_iter (& combined) . find_map (| c | { c . get (1) . unwrap () . as_str () . split (',') . find_map (| f | match f { "apple" => Some ("apple") , "gnustep-1-7" | "gnustep-1-8" | "gnustep-1-9" => Some ("gnustep-old") , "gnustep-2-0" | "gnustep-2-1" => Some ("gnustep") , _ => None , }) }) . unwrap_or ("apple") . to_string () ; result }
};
}
