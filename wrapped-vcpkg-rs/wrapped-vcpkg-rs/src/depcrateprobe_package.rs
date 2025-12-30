// Generated macro for probe_package (function)
macro_rules! Depcrateprobe_package {
() => {
// Module: crate
// Provides: {"probe_package"}
// Dependencies: {}
# [doc = " Deprecated in favor of the find_package function"] # [doc (hidden)] pub fn probe_package (name : & str) -> Result < Library , Error > { Config :: new () . probe (name) }
};
}
