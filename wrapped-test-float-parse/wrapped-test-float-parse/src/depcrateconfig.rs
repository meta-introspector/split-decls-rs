// Generated macro for Config (struct)
macro_rules! DepcrateConfig {
() => {
// Module: crate
// Provides: {"Config"}
// Dependencies: {}
# [doc = " Global configuration."] # [derive (Debug)] pub struct Config { pub timeout : Duration , # [doc = " Failures per test"] pub max_failures : u64 , pub disable_max_failures : bool , # [doc = " If `None`, the default will be used"] pub fuzz_count : Option < u64 > , pub skip_huge : bool , }
};
}
